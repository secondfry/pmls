use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "rustup",
        command: "rustup",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: Some(rustup_version),
        config_paths: &[
            "~/.rustup/settings.toml",
        ],
        env_vars: &[
            "RUSTUP_HOME",
            "RUSTUP_TOOLCHAIN",
            "RUSTUP_DIST_SERVER",
            "RUSTUP_UPDATE_ROOT",
        ],
        packages_dir: Some(|env| {
            if let Some(p) = env.get("RUSTUP_HOME") {
                return Some((p.clone(), "$RUSTUP_HOME"));
            }
            home_dir().map(|h| {
                (std::path::Path::new(&h).join(".rustup").join("toolchains").to_string_lossy().into_owned(), "default")
            })
        }),
        list_cmd: Some(&["rustup", "toolchain", "list"]),
        list_fn: None,
    }
}

/// `rustup --version` → "rustup 1.27.1 (54dd3d00f 2024-04-24)" — extract "1.27.1".
fn rustup_version(output: &str) -> Option<String> {
    output.trim()
        .strip_prefix("rustup ")
        .and_then(|s| s.split_whitespace().next())
        .map(|s| s.to_string())
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
