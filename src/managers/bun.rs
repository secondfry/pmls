use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Bun",
        command: "bun",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: None,
        config_paths: &[
            "~/.bunfig.toml",
            "bunfig.toml",
        ],
        env_vars: &[
            "BUN_INSTALL",
            "BUN_INSTALL_CACHE_DIR",
        ],
        packages_dir: Some(|| {
            std::env::var("BUN_INSTALL").ok().or_else(|| {
                home_dir().map(|h| format!("{}/.bun", h))
            })
        }),
    }
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
