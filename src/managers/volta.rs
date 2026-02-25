use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Volta",
        command: "volta",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: None,
        config_paths: &[
            "~/.volta/",
        ],
        env_vars: &[
            "VOLTA_HOME",
            "VOLTA_FEATURE_PNPM",
        ],
        packages_dir: Some(|env| {
            if let Some(p) = env.get("VOLTA_HOME") {
                return Some((p.clone(), "$VOLTA_HOME"));
            }
            home_dir().map(|h| {
                (std::path::Path::new(&h).join(".volta").to_string_lossy().into_owned(), "default")
            })
        }),
        list_cmd: Some(&["volta", "list", "--format", "plain"]),
        list_fn: None,
    }
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
