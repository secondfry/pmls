use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "pixi",
        command: "pixi",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: Some(pixi_version),
        config_paths: &[
            "~/.config/pixi/config.toml",
            "pixi.toml",
            "pyproject.toml",
        ],
        env_vars: &[
            "PIXI_HOME",
            "PIXI_CACHE_DIR",
            "PIXI_PROJECT_MANIFEST",
            "PIXI_ENVIRONMENT",
        ],
        packages_dir: Some(|env| {
            if let Some(p) = env.get("PIXI_CACHE_DIR") {
                return Some((p.clone(), "$PIXI_CACHE_DIR"));
            }
            if let Some(p) = env.get("PIXI_HOME") {
                return Some((p.clone(), "$PIXI_HOME"));
            }
            home_dir().map(|h| {
                (std::path::Path::new(&h).join(".pixi").to_string_lossy().into_owned(), "default")
            })
        }),
        list_cmd: Some(&["pixi", "list"]),
        list_fn: None,
    }
}

/// `pixi --version` → "pixi 0.39.3" — extract "0.39.3".
fn pixi_version(output: &str) -> Option<String> {
    output.trim()
        .strip_prefix("pixi ")
        .map(|s| s.to_string())
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
