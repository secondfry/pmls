use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "PDM",
        command: "pdm",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: Some(pdm_version),
        config_paths: &[
            "~/.config/pdm/config.toml",
            "pyproject.toml",
            ".pdm.toml",
            ".pdm-python",
        ],
        env_vars: &[
            "PDM_HOME",
            "PDM_CACHE_DIR",
            "PDM_IGNORE_SAVED_PYTHON",
        ],
        packages_dir: Some(|env| {
            if let Some(p) = env.get("PDM_CACHE_DIR") {
                return Some((p.clone(), "$PDM_CACHE_DIR"));
            }
            home_dir().map(|h| {
                #[cfg(windows)]
                let path = std::path::Path::new(&h).join("AppData").join("Local").join("pdm").join("pdm").join("Cache").to_string_lossy().into_owned();
                #[cfg(not(windows))]
                let path = std::path::Path::new(&h).join(".cache").join("pdm").to_string_lossy().into_owned();
                (path, "default")
            })
        }),
        list_cmd: Some(&["pdm", "list"]),
        list_fn: None,
    }
}

/// `pdm --version` → "PDM, version 2.18.1" — extract "2.18.1".
fn pdm_version(output: &str) -> Option<String> {
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
