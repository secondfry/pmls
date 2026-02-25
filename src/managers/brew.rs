use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Homebrew",
        command: "brew",
        category: Category::System,
        version_flag: "--version",
        version_extractor: None,
        config_paths: &[
            "~/.homebrew/",
            "~/.config/brew/",
            "/opt/homebrew/etc/",
            "/usr/local/etc/",
        ],
        env_vars: &[
            "HOMEBREW_PREFIX",
            "HOMEBREW_CELLAR",
            "HOMEBREW_REPOSITORY",
            "HOMEBREW_CACHE",
            "HOMEBREW_NO_AUTO_UPDATE",
        ],
        packages_dir: Some(|env| {
            env.get("HOMEBREW_CELLAR").cloned().or_else(|| {
                env.get("HOMEBREW_PREFIX")
                    .map(|p| format!("{}/Cellar", p))
            })
        }),
        list_cmd: Some(&["brew", "list"]),
    }
}
