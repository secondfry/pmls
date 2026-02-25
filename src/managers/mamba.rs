use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Mamba",
        command: "mamba",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: Some(mamba_version),
        config_paths: &[
            "~/.condarc",
            "~/.mambarc",
        ],
        env_vars: &[
            "MAMBA_ROOT_PREFIX",
            "MAMBA_EXE",
            "CONDA_PREFIX",
            "CONDA_DEFAULT_ENV",
        ],
        packages_dir: Some(|env| {
            if let Some(p) = env.get("MAMBA_ROOT_PREFIX") {
                return Some((
                    std::path::Path::new(p).join("pkgs").to_string_lossy().into_owned(),
                    "$MAMBA_ROOT_PREFIX/pkgs",
                ));
            }
            home_dir().map(|h| {
                (std::path::Path::new(&h).join("mambaforge").join("pkgs").to_string_lossy().into_owned(), "default")
            })
        }),
        list_cmd: Some(&["mamba", "list"]),
        list_fn: None,
    }
}

/// `mamba --version` → "mamba 1.5.8\nconda 24.1.2" — extract "1.5.8".
fn mamba_version(output: &str) -> Option<String> {
    output.lines()
        .find(|l| l.trim_start().starts_with("mamba "))?
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
