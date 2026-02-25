use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Snap",
        command: "snap",
        category: Category::Universal,
        version_flag: "--version",
        version_extractor: None,
        config_paths: &["/etc/snap/"],
        env_vars: &["SNAP", "SNAP_DATA", "SNAP_COMMON", "SNAP_USER_DATA"],
        packages_dir: Some(|_env| Some("/snap".to_string())),
        list_cmd: Some(&["snap", "list"]),
    }
}
