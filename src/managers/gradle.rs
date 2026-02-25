use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Gradle",
        command: "gradle",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: None,
        config_paths: &[
            "~/.gradle/gradle.properties",
            "~/.gradle/init.gradle",
            "build.gradle",
            "build.gradle.kts",
        ],
        env_vars: &[
            "GRADLE_HOME",
            "GRADLE_USER_HOME",
            "GRADLE_OPTS",
        ],
        packages_dir: Some(|env| {
            if let Some(p) = env.get("GRADLE_USER_HOME") {
                return Some((p.clone(), "$GRADLE_USER_HOME"));
            }
            home_dir().map(|h| {
                (std::path::Path::new(&h).join(".gradle").join("caches").to_string_lossy().into_owned(), "default")
            })
        }),
        list_cmd: None,
    }
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
