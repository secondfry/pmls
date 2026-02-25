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
            env.get("GRADLE_USER_HOME").cloned().or_else(|| {
                home_dir().map(|h| format!("{}/.gradle/caches", h))
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
