use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Composer",
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
        packages_dir: Some(|env| {
            env.get("COMPOSER_HOME").cloned().or_else(|| {
                home_dir().map(|h| {
                    std::path::Path::new(&h).join(".composer").join("vendor").to_string_lossy().into_owned()
                })
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
