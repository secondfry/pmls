use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "npm (Node.js)",
        command: "npm",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: None,
        config_paths: &[
            "~/.npmrc",
            "~/.npm/",
        ],
        env_vars: &[
            "NPM_CONFIG_PREFIX",
            "NPM_CONFIG_CACHE",
            "NPM_TOKEN",
            "NODE_PATH",
        ],
        packages_dir: Some(|| {
            std::env::var("NPM_CONFIG_PREFIX").ok().or_else(|| {
                // Common global prefix locations
                #[cfg(windows)]
                return std::env::var("APPDATA")
                    .ok()
                    .map(|p| format!("{}\\npm\\node_modules", p));
                #[cfg(not(windows))]
                return Some("/usr/local/lib/node_modules".to_string());
            })
        }),
    }
}
