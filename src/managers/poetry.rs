use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Poetry",
        command: "poetry",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: Some(poetry_version),
        config_paths: &[
            "~/.config/pypoetry/config.toml",
            "pyproject.toml",
            "poetry.toml",
        ],
        env_vars: &[
            "POETRY_HOME",
            "POETRY_CACHE_DIR",
            "POETRY_VIRTUALENVS_PATH",
            "POETRY_VIRTUALENVS_IN_PROJECT",
        ],
        packages_dir: Some(|env| {
            if let Some(p) = env.get("POETRY_CACHE_DIR") {
                return Some((p.clone(), "$POETRY_CACHE_DIR"));
            }
            home_dir().map(|h| {
                #[cfg(windows)]
                let path = std::path::Path::new(&h).join("AppData").join("Local").join("pypoetry").join("Cache").to_string_lossy().into_owned();
                #[cfg(not(windows))]
                let path = std::path::Path::new(&h).join(".cache").join("pypoetry").to_string_lossy().into_owned();
                (path, "default")
            })
        }),
        list_cmd: Some(&["poetry", "env", "list"]),
        list_fn: None,
    }
}

/// `poetry --version` → "Poetry (version 1.8.3)" — extract "1.8.3".
fn poetry_version(output: &str) -> Option<String> {
    let line = output.lines().find(|l| l.contains("Poetry"))?;
    // Matches "Poetry (version 1.8.3)" or "Poetry version 1.8.3"
    line.split_whitespace()
        .find(|w| w.chars().next().map_or(false, |c| c.is_ascii_digit()))
        .map(|s| s.trim_end_matches(')').to_string())
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
