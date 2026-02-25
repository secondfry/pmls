use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Leiningen",
        command: "lein",
        category: Category::Language,
        version_flag: "version",
        version_extractor: Some(lein_version),
        config_paths: &[
            "~/.lein/profiles.clj",
            "project.clj",
        ],
        env_vars: &[
            "LEIN_HOME",
            "LEIN_JVM_OPTS",
            "LEIN_SNAPSHOTS_IN_RELEASE",
        ],
        packages_dir: Some(|env| {
            if let Some(p) = env.get("LEIN_HOME") {
                return Some((p.clone(), "$LEIN_HOME"));
            }
            home_dir().map(|h| {
                (std::path::Path::new(&h).join(".m2").join("repository").to_string_lossy().into_owned(), "default")
            })
        }),
        list_cmd: None,
        list_fn: None,
    }
}

/// `lein version` → "Leiningen 2.11.2 on Java ..." — extract "2.11.2".
fn lein_version(output: &str) -> Option<String> {
    output.trim()
        .strip_prefix("Leiningen ")
        .and_then(|s| s.split_whitespace().next())
        .map(|s| s.to_string())
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
