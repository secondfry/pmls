use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Advanced Package Tool",
        command: "apt",
        category: Category::System,
        version_flag: "--version",
        version_extractor: None,
        config_paths: &[
            "/etc/apt/apt.conf",
            "/etc/apt/apt.conf.d/",
            "/etc/apt/sources.list",
            "/etc/apt/sources.list.d/",
        ],
        env_vars: &["APT_CONFIG", "DEBIAN_FRONTEND"],
        packages_dir: Some(|_env| Some(("/var/lib/apt/lists".to_string(), "default"))),
        list_cmd: Some(&["apt", "list", "--installed"]),
    }
}
