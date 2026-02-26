use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "mise",
        command: "mise",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: Some(mise_version),
        config_paths: &[
            "~/.config/mise/config.toml",
            "~/.mise.toml",
            ".mise.toml",
            ".mise/config.toml",
        ],
        env_vars: &[
            "MISE_DATA_DIR",
            "MISE_CACHE_DIR",
            "MISE_CONFIG_DIR",
            "MISE_JOBS",
        ],
        packages_dir: Some(|env| {
            if let Some(p) = env.get("MISE_DATA_DIR") {
                return Some((p.clone(), "$MISE_DATA_DIR"));
            }
            home_dir().map(|h| {
                (std::path::Path::new(&h).join(".local").join("share").join("mise").join("installs").to_string_lossy().into_owned(), "default")
            })
        }),
        list_cmd: Some(&["mise", "ls"]),
        list_fn: None,
    }
}

/// `mise --version` → "2024.12.1 macos-arm64 ..." — extract "2024.12.1".
fn mise_version(output: &str) -> Option<String> {
    output.split_whitespace()
        .next()
        .map(|s| s.to_string())
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
