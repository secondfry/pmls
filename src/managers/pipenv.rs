use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Python virtualenv management tool",
        command: "pipenv",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: Some(pipenv_version),
        config_paths: &[
            "Pipfile",
            "Pipfile.lock",
        ],
        env_vars: &[
            "WORKON_HOME",
            "PIPENV_VENV_IN_PROJECT",
            "PIPENV_CACHE_DIR",
        ],
        packages_dir: Some(|env| {
            // WORKON_HOME overrides the default virtualenvs location.
            if let Some(h) = env.get("WORKON_HOME") {
                return Some(h.clone());
            }
            // Default: %USERPROFILE%\.virtualenvs on Windows, ~/.local/share/virtualenvs elsewhere.
            #[cfg(windows)]
            return std::env::var("USERPROFILE").ok().map(|h| {
                std::path::Path::new(&h).join(".virtualenvs").to_string_lossy().into_owned()
            });
            #[cfg(not(windows))]
            return std::env::var("HOME").ok().map(|h| {
                std::path::Path::new(&h)
                    .join(".local").join("share").join("virtualenvs")
                    .to_string_lossy().into_owned()
            });
        }),
        list_cmd: None,
    }
}

/// `pipenv --version` → "pipenv, version 2023.11.15" — extract just "2023.11.15".
fn pipenv_version(output: &str) -> Option<String> {
    output.trim()
        .strip_prefix("pipenv, version ")
        .map(|s| s.trim().to_string())
}
