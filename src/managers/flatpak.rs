use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Flatpak",
        command: "flatpak",
        category: Category::Universal,
        version_flag: "--version",
        version_extractor: None,
        config_paths: &[
            "/etc/flatpak/remotes.d/",
            "~/.local/share/flatpak/",
        ],
        env_vars: &["FLATPAK_USER_DIR", "FLATPAK_SYSTEM_DIR"],
        packages_dir: Some(|_env| Some("/var/lib/flatpak/app".to_string())),
        list_cmd: Some(&["flatpak", "list"]),
    }
}
