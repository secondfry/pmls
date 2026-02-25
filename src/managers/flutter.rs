use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Flutter / Dart pub",
        command: "flutter",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: Some(flutter_version),
        config_paths: &[
            "~/.pub-cache/",
            "pubspec.yaml",
            "pubspec.lock",
        ],
        env_vars: &[
            "FLUTTER_ROOT",
            "PUB_CACHE",
            "PUB_HOSTED_URL",
            "FLUTTER_SUPPRESS_ANALYTICS",
        ],
        packages_dir: Some(|env| {
            if let Some(p) = env.get("PUB_CACHE") {
                return Some((p.clone(), "$PUB_CACHE"));
            }
            home_dir().map(|h| {
                #[cfg(windows)]
                let path = std::path::Path::new(&h).join("AppData").join("Local").join("Pub").join("Cache").to_string_lossy().into_owned();
                #[cfg(not(windows))]
                let path = std::path::Path::new(&h).join(".pub-cache").to_string_lossy().into_owned();
                (path, "default")
            })
        }),
        list_cmd: None,
        list_fn: None,
    }
}

/// `flutter --version` output starts with "Flutter X.Y.Z • channel stable ..."
/// Extract "X.Y.Z".
fn flutter_version(output: &str) -> Option<String> {
    output.lines()
        .find(|l| l.trim_start().starts_with("Flutter "))?
        .split_whitespace()
        .nth(1)
        .map(|s| s.to_string())
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
