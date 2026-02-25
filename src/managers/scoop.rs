use crate::manager::{Category, PackageManager};

/// Extracts the version from `scoop --version` output.
/// Handles two output shapes:
///   interactive:    "b588a06e (HEAD -> master, tag: v0.5.3, ...) ..."  → "v0.5.3"
///   non-interactive "b588a06e chore(release): Bump to version 0.5.3"   → "v0.5.3"
fn scoop_version(output: &str) -> Option<String> {
    for line in output.lines() {
        // Case 1: git log decoration present, e.g. "tag: v0.5.3"
        if let Some(pos) = line.find("tag: ") {
            let after = &line[pos + 5..];
            let ver: String = after
                .chars()
                .take_while(|c| !matches!(c, ',' | ')' | ' '))
                .collect();
            if !ver.is_empty() {
                return Some(ver);
            }
        }
        // Case 2: commit subject contains "version X.Y.Z"
        if let Some(pos) = line.to_lowercase().find("version ") {
            let after = &line[pos + 8..];
            let ver: String = after
                .chars()
                .take_while(|c| c.is_ascii_digit() || *c == '.')
                .collect();
            if ver.contains('.') {
                return Some(format!("v{}", ver));
            }
        }
    }
    // Fallback: second non-empty line as-is
    output
        .lines()
        .filter(|l| !l.trim().is_empty())
        .nth(1)
        .map(|l| l.trim().to_string())
}

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Scoop",
        command: "scoop",
        category: Category::System,
        version_flag: "--version",
        version_extractor: Some(scoop_version),
        config_paths: &[
            "~\\scoop\\config.json",
            "%APPDATA%\\scoop\\config.json",
        ],
        env_vars: &["SCOOP", "SCOOP_GLOBAL"],
        packages_dir: Some(|env| {
            if let Some(root) = env.get("SCOOP") {
                return Some((format!("{}\\apps", root), "$SCOOP"));
            }
            let home = std::env::var("USERPROFILE").unwrap_or_default();
            Some((format!("{}\\scoop\\apps", home), "default"))
        }),
        list_cmd: Some(&["scoop", "list"]),
        list_fn: None,
    }
}
