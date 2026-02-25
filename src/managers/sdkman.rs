use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "SDKMAN!",
        command: "sdk",
        category: Category::Language,
        version_flag: "version",
        version_extractor: Some(sdk_version),
        config_paths: &[
            "~/.sdkman/etc/config",
        ],
        env_vars: &[
            "SDKMAN_DIR",
            "SDKMAN_CANDIDATES_DIR",
            "SDKMAN_CANDIDATES_API",
        ],
        packages_dir: Some(|env| {
            if let Some(p) = env.get("SDKMAN_CANDIDATES_DIR") {
                return Some((p.clone(), "$SDKMAN_CANDIDATES_DIR"));
            }
            if let Some(p) = env.get("SDKMAN_DIR") {
                return Some((
                    std::path::Path::new(p).join("candidates").to_string_lossy().into_owned(),
                    "$SDKMAN_DIR/candidates",
                ));
            }
            home_dir().map(|h| {
                (std::path::Path::new(&h).join(".sdkman").join("candidates").to_string_lossy().into_owned(), "default")
            })
        }),
        list_cmd: Some(&["sdk", "list"]),
        list_fn: None,
    }
}

/// `sdk version` → "SDKMAN 5.18.2" — extract "5.18.2".
fn sdk_version(output: &str) -> Option<String> {
    output.lines()
        .find(|l| l.to_uppercase().contains("SDKMAN"))
        .and_then(|l| l.split_whitespace().last())
        .map(|s| s.to_string())
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
