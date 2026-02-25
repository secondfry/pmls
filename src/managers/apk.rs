use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Alpine Package Keeper",
        command: "apk",
        category: Category::System,
        version_flag: "--version",
        version_extractor: None,
        config_paths: &["/etc/apk/world", "/etc/apk/repositories"],
        env_vars: &[],
        packages_dir: Some(|_env| Some(("/lib/apk/db".to_string(), "default"))),
        list_cmd: Some(&["apk", "list", "--installed"]),
        list_fn: None,
    }
}
