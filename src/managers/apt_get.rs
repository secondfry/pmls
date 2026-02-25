use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "APT-GET",
        command: "apt-get",
        category: Category::System,
        version_flag: "--version",
        version_extractor: None,
        config_paths: &[
            "/etc/apt/apt.conf",
            "/etc/apt/apt.conf.d/",
            "/etc/apt/sources.list",
        ],
        env_vars: &["APT_CONFIG", "DEBIAN_FRONTEND"],
        packages_dir: Some(|| Some("/var/cache/apt/archives".to_string())),
    }
}
