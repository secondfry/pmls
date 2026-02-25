use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "pyenv",
        command: "pyenv",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: Some(pyenv_version),
        config_paths: &[
            "~/.pyenv/",
            "~/.python-version",
            ".python-version",
        ],
        env_vars: &[
            "PYENV_ROOT",
            "PYENV_VERSION",
            "PYENV_SHELL",
        ],
        packages_dir: Some(|env| {
            if let Some(p) = env.get("PYENV_ROOT") {
                return Some((
                    std::path::Path::new(p).join("versions").to_string_lossy().into_owned(),
                    "$PYENV_ROOT/versions",
                ));
            }
            home_dir().map(|h| {
                (std::path::Path::new(&h).join(".pyenv").join("versions").to_string_lossy().into_owned(), "default")
            })
        }),
        list_cmd: Some(&["pyenv", "versions"]),
        list_fn: None,
    }
}

/// `pyenv --version` → "pyenv 2.4.1" — extract "2.4.1".
fn pyenv_version(output: &str) -> Option<String> {
    output.trim()
        .strip_prefix("pyenv ")
        .map(|s| s.to_string())
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
