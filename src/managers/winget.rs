use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Windows Package Manager",
        command: "winget",
        category: Category::System,
        version_flag: "--version",
        version_extractor: None,
        config_paths: &[
            "%LOCALAPPDATA%\\Microsoft\\WinGet\\Settings\\settings.json",
        ],
        env_vars: &[],
        packages_dir: Some(|_env| {
            std::env::var("LOCALAPPDATA")
                .ok()
                .map(|p| (format!("{}\\Microsoft\\WinGet\\Packages", p), "$LOCALAPPDATA"))
        }),
        list_cmd: Some(&["winget", "list", "--disable-interactivity", "--accept-source-agreements"]),
        list_fn: None,
    }
}
