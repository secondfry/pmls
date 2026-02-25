use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Cabal (Haskell)",
        command: "cabal",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: Some(cabal_version),
        config_paths: &[
            "~/.cabal/config",
            "cabal.project",
            "*.cabal",
        ],
        env_vars: &[
            "CABAL_DIR",
            "CABAL_CONFIG",
        ],
        packages_dir: Some(|env| {
            if let Some(p) = env.get("CABAL_DIR") {
                return Some((p.clone(), "$CABAL_DIR"));
            }
            home_dir().map(|h| {
                (std::path::Path::new(&h).join(".cabal").join("store").to_string_lossy().into_owned(), "default")
            })
        }),
        list_cmd: Some(&["cabal", "list", "--installed"]),
        list_fn: None,
    }
}

/// `cabal --version` → "cabal-install version 3.12.1.0\ncompiled using version ..."
/// Extract "3.12.1.0".
fn cabal_version(output: &str) -> Option<String> {
    let line = output.lines().find(|l| l.contains("cabal-install version"))?;
    line.split("version ").nth(1)
        .and_then(|s| s.split_whitespace().next())
        .map(|s| s.to_string())
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
