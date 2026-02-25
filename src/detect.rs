use std::process::Command;

use crate::manager::{DetectedPackageManager, PackageManager};

/// Returns `true` if the given command is reachable on `PATH`.
pub fn command_exists(cmd: &str) -> bool {
    #[cfg(windows)]
    let result = Command::new("where").arg(cmd).output();
    #[cfg(not(windows))]
    let result = Command::new("which").arg(cmd).output();

    result.map(|o| o.status.success()).unwrap_or(false)
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

/// Probes PATH for every known package manager and returns the detected ones.
pub fn detect(all: Vec<PackageManager>) -> Vec<DetectedPackageManager> {
    all.into_iter()
        .filter_map(|pm| {
            if command_exists(pm.command) {
                let version = get_version(pm.command, pm.version_flag, pm.version_extractor);
                let packages_dir = pm.packages_dir.and_then(|f| f());
                Some(DetectedPackageManager { manager: pm, version, packages_dir })
            } else {
                None
            }
        })
        .collect()
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
