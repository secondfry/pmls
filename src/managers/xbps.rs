use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "X Binary Package System",
        command: "xbps-install",
        category: Category::System,
        version_flag: "--version",
        version_extractor: None,
        config_paths: &["/etc/xbps.d/", "/usr/share/xbps.d/"],
        env_vars: &["XBPS_TARGET_ARCH"],
        packages_dir: Some(|_env| Some("/var/db/xbps".to_string())),
        list_cmd: Some(&["xbps-query", "-l"]),
    }
}
