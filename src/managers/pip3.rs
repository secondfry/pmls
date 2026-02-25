use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "pip3 (Python)",
        command: "pip3",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: None,
        config_paths: &[
            "~/.pip/pip.conf",
            "~/.config/pip/pip.conf",
            "%APPDATA%\\pip\\pip.ini",
        ],
        env_vars: &[
            "PIP_INDEX_URL",
            "PIP_EXTRA_INDEX_URL",
            "PIP_CACHE_DIR",
            "PIP_CONFIG_FILE",
            "VIRTUAL_ENV",
        ],
        packages_dir: Some(|| {
            std::env::var("VIRTUAL_ENV")
                .ok()
                .map(|v| {
                    #[cfg(windows)]
                    return format!("{}\\Lib\\site-packages", v);
                    #[cfg(not(windows))]
                    return format!("{}/lib/python/site-packages", v);
                })
        }),
        list_cmd: Some(&["pip3", "list"]),
    }
}
