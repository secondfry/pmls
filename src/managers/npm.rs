use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Node Package Manager",
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
        packages_dir: Some(|env| {
            if let Some(p) = env.get("NPM_CONFIG_PREFIX") {
                return Some((p.clone(), "$NPM_CONFIG_PREFIX"));
            }
            #[cfg(windows)]
            return std::env::var("APPDATA")
                .ok()
                .map(|p| (format!("{}\\npm\\node_modules", p), "default"));
            #[cfg(not(windows))]
            return Some(("/usr/local/lib/node_modules".to_string(), "default"));
        }),
        list_cmd: Some(&["npm", "-g", "ls", "--depth=0"]),
    }
}
