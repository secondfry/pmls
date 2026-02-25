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
            if let Some(p) = env.get("COMPOSER_HOME") {
                return Some((p.clone(), "$COMPOSER_HOME"));
            }
            home_dir().map(|h| {
                (std::path::Path::new(&h).join(".composer").join("vendor").to_string_lossy().into_owned(), "default")
            })
        }),
        list_cmd: Some(&["composer", "global", "show"]),
        list_fn: None,
    }
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
