use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Composer (PHP)",
        command: "composer",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: None,
        config_paths: &[
            "~/.composer/config.json",
            "~/.config/composer/config.json",
            "composer.json",
        ],
        env_vars: &[
            "COMPOSER_HOME",
            "COMPOSER_CACHE_DIR",
            "COMPOSER_VENDOR_DIR",
            "COMPOSER_AUTH",
        ],
        packages_dir: Some(|| {
            std::env::var("COMPOSER_HOME").ok().or_else(|| {
                home_dir().map(|h| format!("{}/.composer/vendor", h))
            })
        }),
        list_cmd: Some(&["composer", "global", "show"]),
    }
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
