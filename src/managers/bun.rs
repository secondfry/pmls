use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Bun",
        command: "bun",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: None,
        config_paths: &[
            "~/.bunfig.toml",
            "bunfig.toml",
        ],
        env_vars: &[
            "BUN_INSTALL",
            "BUN_INSTALL_CACHE_DIR",
        ],
        packages_dir: Some(|env| {
            if let Some(p) = env.get("BUN_INSTALL") {
                return Some((p.clone(), "$BUN_INSTALL"));
            }
            home_dir().map(|h| {
                (std::path::Path::new(&h).join(".bun").to_string_lossy().into_owned(), "default")
            })
        }),
        list_cmd: Some(&["bun", "pm", "-g", "ls"]),
    }
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
