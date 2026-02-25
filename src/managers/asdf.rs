use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "asdf",
        command: "asdf",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: Some(asdf_version),
        config_paths: &[
            "~/.asdfrc",
            "~/.tool-versions",
            ".tool-versions",
        ],
        env_vars: &[
            "ASDF_DIR",
            "ASDF_DATA_DIR",
            "ASDF_CONFIG_FILE",
            "ASDF_DEFAULT_TOOL_VERSIONS_FILENAME",
        ],
        packages_dir: Some(|env| {
            if let Some(p) = env.get("ASDF_DATA_DIR") {
                return Some((p.clone(), "$ASDF_DATA_DIR"));
            }
            if let Some(p) = env.get("ASDF_DIR") {
                return Some((p.clone(), "$ASDF_DIR"));
            }
            home_dir().map(|h| {
                (std::path::Path::new(&h).join(".asdf").join("installs").to_string_lossy().into_owned(), "default")
            })
        }),
        list_cmd: Some(&["asdf", "list"]),
        list_fn: None,
    }
}

/// `asdf --version` → "v0.14.1-ccdd47d" — extract "v0.14.1".
fn asdf_version(output: &str) -> Option<String> {
    let raw = output.trim();
    // strip leading 'v' split on '-' to drop commit hash
    let without_v = raw.strip_prefix('v').unwrap_or(raw);
    Some(without_v.split('-').next().unwrap_or(without_v).to_string())
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
