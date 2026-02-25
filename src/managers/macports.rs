use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "MacPorts",
        command: "port",
        category: Category::System,
        version_flag: "version",
        version_extractor: None,
        config_paths: &[
            "/opt/local/etc/macports/macports.conf",
            "/opt/local/etc/macports/sources.conf",
        ],
        env_vars: &[],
        packages_dir: Some(|_env| Some(("/opt/local/var/macports/registry".to_string(), "default"))),
        list_cmd: Some(&["port", "installed"]),
        list_fn: None,
    }
}
