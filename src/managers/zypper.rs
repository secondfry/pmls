use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Zypper",
        command: "zypper",
        category: Category::System,
        version_flag: "--version",
        version_extractor: None,
        config_paths: &["/etc/zypp/zypp.conf", "/etc/zypp/repos.d/"],
        env_vars: &[],
        packages_dir: Some(|_env| Some(("/var/cache/zypp".to_string(), "default"))),
        list_cmd: Some(&["zypper", "packages", "--installed-only"]),
        list_fn: None,
    }
}
