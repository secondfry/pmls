use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "bin",
        command: "bin",
        category: Category::System,
        version_flag: "--version",
        version_extractor: Some(bin_version),
        config_paths: &[
            "~/.config/bin/config.toml",
        ],
        env_vars: &[
            "BIN_PATH",
        ],
        packages_dir: Some(|| {
            std::env::var("BIN_PATH").ok().or_else(|| {
                home_dir().map(|h| {
                    #[cfg(windows)]
                    return format!("{}\\.local\\bin", h);
                    #[cfg(not(windows))]
                    return format!("{}/.local/bin", h);
                })
            })
        }),
    }
}

/// `bin --version` outputs multiple lines; extract the "bin version X.Y.Z" line.
///   "bin version 0.23.1"
///   "commit: ..."
///   ...
fn bin_version(output: &str) -> Option<String> {
    output
        .lines()
        .find(|l| l.trim().starts_with("bin version"))
        .map(|l| l.trim().to_string())
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
