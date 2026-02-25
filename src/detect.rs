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
