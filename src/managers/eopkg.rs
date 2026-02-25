use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Eopkg",
        command: "eopkg",
        category: Category::System,
        version_flag: "--version",
        version_extractor: None,
        config_paths: &["/etc/eopkg/eopkg.conf"],
        env_vars: &[],
        packages_dir: Some(|_env| Some(("/var/lib/eopkg/package".to_string(), "default"))),
        list_cmd: Some(&["eopkg", "list-installed"]),
        list_fn: None,
    }
}
