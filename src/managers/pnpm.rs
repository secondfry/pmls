use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "pnpm",
        command: "pnpm",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: None,
        config_paths: &[
            "~/.npmrc",
            "~/.pnpmfile.cjs",
        ],
        env_vars: &[
            "PNPM_HOME",
            "NPM_CONFIG_REGISTRY",
        ],
        packages_dir: Some(|| {
            std::env::var("PNPM_HOME").ok()
        }),
    }
}
