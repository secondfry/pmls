use std::process::Command;

use rayon::prelude::*;

use crate::manager::{DetectedPackageManager, EnvMap, PackageManager};

/// Returns `true` if the given command is reachable on `PATH`.
/// Uses an in-process lookup via the `which` crate rather than spawning a
/// `where`/`which` subprocess, which is meaningfully faster at scale.
pub fn command_exists(cmd: &str) -> bool {
    which::which(cmd).is_ok()
}

/// Runs `cmd <flag>` and returns a version string.
/// On Windows all commands are dispatched through `cmd /C` so that `.cmd`/`.bat`
/// wrappers (e.g. npm, yarn, pnpm) are resolved correctly.
pub fn get_version(
    cmd: &str,
    flag: &str,
    extractor: Option<fn(&str) -> Option<String>>,
) -> Option<String> {
    #[cfg(windows)]
    let output = Command::new("cmd").args(["/C", cmd, flag]).output().ok()?;
    #[cfg(not(windows))]
    let output = Command::new(cmd).arg(flag).output().ok()?;

    // Some tools write version to stderr (e.g. certain Python builds).
    let raw = if output.stdout.is_empty() {
        output.stderr
    } else {
        output.stdout
    };
    let text = String::from_utf8_lossy(&raw);
    let text = text.as_ref();

    if let Some(f) = extractor {
        return f(text);
    }

    text.lines()
        .find(|l| !l.trim().is_empty())
        .map(|l| l.trim().to_string())
}

/// Probe and detect a single manager, returning `Some` if it exists on PATH.
fn probe(pm: PackageManager) -> Option<DetectedPackageManager> {
    if !command_exists(pm.command) {
        return None;
    }
    let version = get_version(pm.command, pm.version_flag, pm.version_extractor);
    let env_map: EnvMap = pm.env_vars.iter()
        .filter_map(|k| std::env::var(k).ok().map(|v| (*k, v)))
        .collect();
    let (packages_dir, packages_dir_source) = match pm.packages_dir.and_then(|f| f(&env_map)) {
        Some((path, src)) => (Some(path), Some(src)),
        None => (None, None),
    };
    Some(DetectedPackageManager { manager: pm, version, packages_dir, packages_dir_source })
}

/// Probes PATH for every known package manager and returns the detected ones.
/// Detection is parallelised across all managers using rayon — each manager's
/// PATH probe and version subprocess run concurrently on the thread pool.
/// Use `detect_grouped` when you need results bucketed by category label.
#[allow(dead_code)]
pub fn detect(all: Vec<PackageManager>) -> Vec<DetectedPackageManager> {
    all.into_par_iter().filter_map(probe).collect()
}

/// Detects all managers in one parallel pass over every group at once, then
/// re-buckets results into the original group order.
///
/// This is faster than calling `detect()` per group sequentially because the
/// rayon thread pool sees the full set of 77+ managers simultaneously and
/// can schedule their subprocesses optimally across all CPU cores.
pub fn detect_grouped(
    groups: Vec<(&'static str, Vec<PackageManager>)>,
) -> Vec<(&'static str, Vec<DetectedPackageManager>)> {
    // Flatten to (group_index, manager) so we can re-bucket after parallel detection.
    let labeled: Vec<(usize, PackageManager)> = groups
        .iter()
        .enumerate()
        .flat_map(|(i, (_, managers))| managers.iter().map(move |pm| (i, pm.clone())))
        .collect();

    // Single parallel pass over all managers across all groups.
    let mut detected: Vec<(usize, DetectedPackageManager)> = labeled
        .into_par_iter()
        .filter_map(|(idx, pm)| probe(pm).map(|d| (idx, d)))
        .collect();

    // Stable sort keeps managers within each group in their original order.
    detected.sort_by_key(|(i, _)| *i);

    // Re-bucket by group index, preserving group order.
    let mut result: Vec<(&'static str, Vec<DetectedPackageManager>)> =
        groups.iter().map(|(label, _)| (*label, Vec::new())).collect();
    for (idx, d) in detected {
        result[idx].1.push(d);
    }
    result
}

/// Runs the given list command and returns its output lines.
///
/// On Windows the command is dispatched through `cmd /C` so that `.cmd`/`.bat`
/// shims are resolved.  Returns `Ok(lines)` on success or `Err(stderr)` on
/// failure / non-zero exit.
pub fn run_list(cmd: &[&'static str]) -> Result<Vec<String>, String> {
    let (exe, args) = cmd.split_first().expect("list_cmd must not be empty");

    #[cfg(windows)]
    let output = {
        let mut full_args: Vec<&str> = vec!["/C", exe];
        full_args.extend_from_slice(args);
        Command::new("cmd").args(&full_args).output()
    };
    #[cfg(not(windows))]
    let output = Command::new(exe).args(args).output();

    let output = output.map_err(|e| e.to_string())?;

    if output.status.success() {
        let text = String::from_utf8_lossy(&output.stdout);
        Ok(text
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| l.to_string())
            .collect())
    } else {
        let err = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(if err.is_empty() {
            format!("exited with {}", output.status)
        } else {
            err
        })
    }
}
