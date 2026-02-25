use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Rust package manager",
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
        packages_dir: Some(|env| {
            env.get("CARGO_HOME").cloned().or_else(|| {
                home_dir().map(|p| {
                    std::path::Path::new(&p).join(".cargo").join("bin").to_string_lossy().into_owned()
                })
            })
        }),
        list_cmd: Some(&["cargo", "install", "--list"]),
    }
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
