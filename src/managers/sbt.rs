use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "sbt",
        command: "sbt",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: Some(sbt_version),
        config_paths: &[
            "~/.sbt/",
            "build.sbt",
            "project/build.properties",
        ],
        env_vars: &[
            "SBT_HOME",
            "SBT_OPTS",
            "SBT_CREDENTIALS",
        ],
        packages_dir: Some(|_env| {
            home_dir().map(|h| {
                (std::path::Path::new(&h).join(".ivy2").join("cache").to_string_lossy().into_owned(), "default")
            })
        }),
        list_cmd: None,
        list_fn: None,
    }
}

/// `sbt --version` → "sbt version in this project: 1.10.4\nsbt script version: 1.10.4" — extract "1.10.4".
fn sbt_version(output: &str) -> Option<String> {
    for line in output.lines() {
        let lower = line.to_lowercase();
        if lower.contains("script version") || lower.contains("sbt version") {
            if let Some(v) = line.split(':').nth(1) {
                let trimmed = v.trim().to_string();
                if !trimmed.is_empty() {
                    return Some(trimmed);
                }
            }
        }
    }
    None
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
