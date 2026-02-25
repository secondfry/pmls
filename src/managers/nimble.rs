use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Nimble (Nim)",
        command: "nimble",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: Some(nimble_version),
        config_paths: &[
            "~/.nimble/",
            "*.nimble",
        ],
        env_vars: &[
            "NIMBLE_DIR",
        ],
        packages_dir: Some(|env| {
            if let Some(p) = env.get("NIMBLE_DIR") {
                return Some((p.clone(), "$NIMBLE_DIR"));
            }
            home_dir().map(|h| {
                (std::path::Path::new(&h).join(".nimble").join("pkgs2").to_string_lossy().into_owned(), "default")
            })
        }),
        list_cmd: Some(&["nimble", "list", "--installed"]),
        list_fn: None,
    }
}

/// `nimble --version` → "nimble v0.16.4 compiled at 2024-10-02 ..." — extract "0.16.4".
fn nimble_version(output: &str) -> Option<String> {
    output.trim()
        .strip_prefix("nimble v")
        .and_then(|s| s.split_whitespace().next())
        .map(|s| s.to_string())
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
