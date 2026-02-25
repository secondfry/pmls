use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Fast Node Manager",
        command: "fnm",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: Some(fnm_version),
        config_paths: &[
            "~/.config/fnm/",
            ".node-version",
            ".nvmrc",
        ],
        env_vars: &[
            "FNM_DIR",
            "FNM_NODE_DIST_MIRROR",
            "FNM_MULTISHELL_PATH",
        ],
        packages_dir: Some(|env| {
            if let Some(p) = env.get("FNM_DIR") {
                return Some((p.clone(), "$FNM_DIR"));
            }
            #[cfg(windows)]
            return std::env::var("APPDATA").ok().map(|h| {
                (std::path::Path::new(&h).join("fnm").to_string_lossy().into_owned(), "default")
            });
            #[cfg(not(windows))]
            return std::env::var("HOME").ok().map(|h| {
                (std::path::Path::new(&h).join(".local").join("share").join("fnm").to_string_lossy().into_owned(), "default")
            });
        }),
        list_cmd: Some(&["fnm", "list"]),
        list_fn: None,
    }
}

/// `fnm --version` → "fnm 1.35.1" — extract just "1.35.1".
fn fnm_version(output: &str) -> Option<String> {
    output.trim()
        .strip_prefix("fnm ")
        .map(|s| s.trim().to_string())
}
