use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "rbenv",
        command: "rbenv",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: Some(rbenv_version),
        config_paths: &[
            "~/.rbenv/",
            "~/.ruby-version",
            ".ruby-version",
        ],
        env_vars: &[
            "RBENV_ROOT",
            "RBENV_VERSION",
            "RBENV_SHELL",
        ],
        packages_dir: Some(|env| {
            if let Some(p) = env.get("RBENV_ROOT") {
                return Some((
                    std::path::Path::new(p).join("versions").to_string_lossy().into_owned(),
                    "$RBENV_ROOT/versions",
                ));
            }
            home_dir().map(|h| {
                (std::path::Path::new(&h).join(".rbenv").join("versions").to_string_lossy().into_owned(), "default")
            })
        }),
        list_cmd: Some(&["rbenv", "versions"]),
        list_fn: None,
    }
}

/// `rbenv --version` → "rbenv 1.3.0" — extract "1.3.0".
fn rbenv_version(output: &str) -> Option<String> {
    output.trim()
        .strip_prefix("rbenv ")
        .and_then(|s| s.split_whitespace().next())
        .map(|s| s.to_string())
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
