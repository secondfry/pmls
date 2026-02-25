use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "goenv",
        command: "goenv",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: Some(goenv_version),
        config_paths: &[
            "~/.goenv/",
            "~/.go-version",
            ".go-version",
        ],
        env_vars: &[
            "GOENV_ROOT",
            "GOENV_VERSION",
            "GOENV_SHELL",
        ],
        packages_dir: Some(|env| {
            if let Some(p) = env.get("GOENV_ROOT") {
                return Some((
                    std::path::Path::new(p).join("versions").to_string_lossy().into_owned(),
                    "$GOENV_ROOT/versions",
                ));
            }
            home_dir().map(|h| {
                (std::path::Path::new(&h).join(".goenv").join("versions").to_string_lossy().into_owned(), "default")
            })
        }),
        list_cmd: Some(&["goenv", "versions"]),
        list_fn: None,
    }
}

/// `goenv --version` → "goenv 2.1.20" — extract "2.1.20".
fn goenv_version(output: &str) -> Option<String> {
    output.trim()
        .strip_prefix("goenv ")
        .and_then(|s| s.split_whitespace().next())
        .map(|s| s.to_string())
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
