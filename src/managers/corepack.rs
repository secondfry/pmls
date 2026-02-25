use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Corepack",
        command: "corepack",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: None,
        config_paths: &[
            "package.json",
        ],
        env_vars: &[
            "COREPACK_HOME",
            "COREPACK_NPM_REGISTRY",
            "COREPACK_ENABLE_STRICT",
        ],
        packages_dir: Some(|env| {
            if let Some(p) = env.get("COREPACK_HOME") {
                return Some((p.clone(), "$COREPACK_HOME"));
            }
            home_dir().map(|h| {
                #[cfg(windows)]
                let path = std::path::Path::new(&h).join("AppData").join("Local").join("node").join("corepack").to_string_lossy().into_owned();
                #[cfg(not(windows))]
                let path = std::path::Path::new(&h).join(".cache").join("node").join("corepack").to_string_lossy().into_owned();
                (path, "default")
            })
        }),
        list_cmd: None,
        list_fn: None,
    }
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
