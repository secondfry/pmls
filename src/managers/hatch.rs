use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Hatch",
        command: "hatch",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: Some(hatch_version),
        config_paths: &[
            "~/.config/hatch/config.toml",
            "pyproject.toml",
            "hatch.toml",
        ],
        env_vars: &[
            "HATCH_ENV_TYPE_VIRTUAL_PATH",
            "HATCH_DATA_DIR",
            "HATCH_CACHE_DIR",
            "HATCH_CONFIG",
        ],
        packages_dir: Some(|env| {
            if let Some(p) = env.get("HATCH_DATA_DIR") {
                return Some((p.clone(), "$HATCH_DATA_DIR"));
            }
            home_dir().map(|h| {
                #[cfg(windows)]
                let path = std::path::Path::new(&h).join("AppData").join("Local").join("hatch").to_string_lossy().into_owned();
                #[cfg(not(windows))]
                let path = std::path::Path::new(&h).join(".local").join("share").join("hatch").to_string_lossy().into_owned();
                (path, "default")
            })
        }),
        list_cmd: Some(&["hatch", "env", "show"]),
        list_fn: None,
    }
}

/// `hatch --version` → "Hatch, version 1.12.0" — extract "1.12.0".
fn hatch_version(output: &str) -> Option<String> {
    output.trim()
        .split(", version ")
        .nth(1)
        .map(|s| s.to_string())
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
