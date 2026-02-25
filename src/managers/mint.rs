use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Mint (Swift)",
        command: "mint",
        category: Category::Language,
        version_flag: "version",
        version_extractor: Some(mint_version),
        config_paths: &[
            "Mintfile",
            "~/.mint/",
        ],
        env_vars: &[
            "MINT_PATH",
            "MINT_LINK_PATH",
        ],
        packages_dir: Some(|env| {
            if let Some(p) = env.get("MINT_PATH") {
                return Some((p.clone(), "$MINT_PATH"));
            }
            Some(("/usr/local/lib/mint".to_string(), "default"))
        }),
        list_cmd: Some(&["mint", "list"]),
        list_fn: None,
    }
}

/// `mint version` → "Version: 0.17.5" — extract "0.17.5".
fn mint_version(output: &str) -> Option<String> {
    output.lines()
        .find(|l| l.to_lowercase().contains("version"))?
        .split(':')
        .nth(1)
        .map(|s| s.trim().to_string())
}
