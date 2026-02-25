use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Pacman",
        command: "pacman",
        category: Category::System,
        version_flag: "--version",
        version_extractor: None,
        config_paths: &["/etc/pacman.conf", "/etc/pacman.d/mirrorlist"],
        env_vars: &[],
        packages_dir: Some(|| Some("/var/lib/pacman/local".to_string())),
    }
}
