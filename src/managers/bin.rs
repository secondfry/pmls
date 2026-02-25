use serde::Deserialize;

use crate::manager::{Category, PackageManager};

#[derive(Deserialize)]
struct BinConfig {
    default_path: Option<String>,
}

pub fn manager() -> PackageManager {
    PackageManager {
        name: "bin",
        command: "bin",
        category: Category::System,
        version_flag: "--version",
        version_extractor: Some(bin_version),
        config_paths: &[
            "~/.config/bin/config.json",
        ],
        env_vars: &[
            "BIN_PATH",
        ],
        packages_dir: Some(|| {
            if let Ok(p) = std::env::var("BIN_PATH") {
                return Some(p);
            }
            let config_path = {
                #[cfg(windows)]
                {
                    std::env::var("USERPROFILE")
                        .ok()
                        .map(|h| format!("{}\\.config\\bin\\config.json", h))
                }
                #[cfg(not(windows))]
                {
                    std::env::var("HOME")
                        .ok()
                        .map(|h| format!("{}/.config/bin/config.json", h))
                }
            }?;
            let mut bytes = std::fs::read(config_path).ok()?;
            let config: BinConfig = simd_json::from_slice(&mut bytes).ok()?;
            config.default_path
        }),
        list_cmd: Some(&["bin", "ls"]),
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
        .and_then(|l| l.trim().strip_prefix("bin version"))
        .map(|v| v.trim().to_string())
}
