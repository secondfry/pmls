use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Yellowdog Updater Modified",
        command: "yum",
        category: Category::System,
        version_flag: "--version",
        version_extractor: None,
        config_paths: &["/etc/yum.conf", "/etc/yum.repos.d/"],
        env_vars: &[],
        packages_dir: Some(|| Some("/var/cache/yum".to_string())),
        list_cmd: Some(&["yum", "list", "installed"]),
    }
}
