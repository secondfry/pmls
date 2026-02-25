use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "nodenv",
        command: "nodenv",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: Some(nodenv_version),
        config_paths: &[
            "~/.nodenv/",
            "~/.node-version",
            ".node-version",
        ],
        env_vars: &[
            "NODENV_ROOT",
            "NODENV_VERSION",
            "NODENV_SHELL",
        ],
        packages_dir: Some(|env| {
            if let Some(p) = env.get("NODENV_ROOT") {
                return Some((
                    std::path::Path::new(p).join("versions").to_string_lossy().into_owned(),
                    "$NODENV_ROOT/versions",
                ));
            }
            home_dir().map(|h| {
                (std::path::Path::new(&h).join(".nodenv").join("versions").to_string_lossy().into_owned(), "default")
            })
        }),
        list_cmd: Some(&["nodenv", "versions"]),
        list_fn: None,
    }
}

/// `nodenv --version` → "nodenv 1.4.1" — extract "1.4.1".
fn nodenv_version(output: &str) -> Option<String> {
    output.trim()
        .strip_prefix("nodenv ")
        .and_then(|s| s.split_whitespace().next())
        .map(|s| s.to_string())
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
