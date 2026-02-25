use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Coursier",
        command: "cs",
        category: Category::Language,
        version_flag: "version",
        version_extractor: None,
        config_paths: &[
            "~/.config/coursier/",
        ],
        env_vars: &[
            "COURSIER_CACHE",
            "COURSIER_JVM_CACHE",
        ],
        packages_dir: Some(|env| {
            if let Some(p) = env.get("COURSIER_CACHE") {
                return Some((p.clone(), "$COURSIER_CACHE"));
            }
            home_dir().map(|h| {
                #[cfg(windows)]
                let path = std::path::Path::new(&h).join("AppData").join("Local").join("Coursier").join("cache").to_string_lossy().into_owned();
                #[cfg(target_os = "macos")]
                let path = std::path::Path::new(&h).join("Library").join("Caches").join("Coursier").join("v1").to_string_lossy().into_owned();
                #[cfg(all(not(windows), not(target_os = "macos")))]
                let path = std::path::Path::new(&h).join(".cache").join("coursier").join("v1").to_string_lossy().into_owned();
                (path, "default")
            })
        }),
        list_cmd: Some(&["cs", "list"]),
        list_fn: None,
    }
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
