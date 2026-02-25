use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Cargo (Rust)",
        command: "cargo",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: None,
        config_paths: &[
            "~/.cargo/config.toml",
            "~/.cargo/config",
        ],
        env_vars: &[
            "CARGO_HOME",
            "CARGO_TARGET_DIR",
            "RUSTUP_HOME",
            "CARGO_INCREMENTAL",
        ],
        packages_dir: Some(|| {
            std::env::var("CARGO_HOME").ok().or_else(|| {
                home_dir().map(|p| format!("{}/.cargo/bin", p))
            })
        }),
    }
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
