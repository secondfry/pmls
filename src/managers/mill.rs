use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Mill",
        command: "mill",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: Some(mill_version),
        config_paths: &[
            "build.mill",
            "build.sc",
            ".mill-version",
        ],
        env_vars: &[
            "MILL_JVM_OPTS",
            "MILL_OUTPUT_DIR",
        ],
        packages_dir: Some(|_env| {
            home_dir().map(|h| {
                (std::path::Path::new(&h).join(".cache").join("mill").to_string_lossy().into_owned(), "default")
            })
        }),
        list_cmd: None,
        list_fn: None,
    }
}

/// `mill --version` → "Mill Build Tool version 0.11.12\n..." — extract "0.11.12".
fn mill_version(output: &str) -> Option<String> {
    output.lines()
        .find(|l| l.to_lowercase().contains("version"))?
        .split_whitespace()
        .last()
        .map(|s| s.to_string())
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
