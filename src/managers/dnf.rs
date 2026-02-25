use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "DNF",
        command: "dnf",
        category: Category::System,
        version_flag: "--version",
        version_extractor: None,
        config_paths: &["/etc/dnf/dnf.conf", "/etc/dnf/plugins/", "/etc/yum.repos.d/"],
        env_vars: &[],
        packages_dir: Some(|| Some("/var/cache/dnf".to_string())),
        list_cmd: Some(&["dnf", "list", "installed"]),
    }
}
