use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Chocolatey",
        command: "choco",
        category: Category::System,
        version_flag: "--version",
        version_extractor: None,
        config_paths: &[
            "%ChocolateyInstall%\\config\\chocolatey.config",
            "%ALLUSERSPROFILE%\\chocolatey\\config\\chocolatey.config",
        ],
        env_vars: &[
            "ChocolateyInstall",
            "ChocolateyBinRoot",
            "ChocolateyLastPathUpdate",
        ],
        packages_dir: Some(|env| {
            env.get("ChocolateyInstall")
                .map(|p| (format!("{}\\lib", p), "$ChocolateyInstall"))
        }),
        list_cmd: Some(&["choco", "list", "--local-only"]),
        list_fn: None,
    }
}
