use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Deno",
        command: "deno",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: Some(deno_version),
        config_paths: &[
            "deno.json",
            "deno.jsonc",
            "import_map.json",
        ],
        env_vars: &[
            "DENO_DIR",
            "DENO_INSTALL_ROOT",
            "DENO_NO_UPDATE_CHECK",
            "NPM_CONFIG_REGISTRY",
        ],
        packages_dir: Some(|env| {
            if let Some(p) = env.get("DENO_DIR") {
                return Some((p.clone(), "$DENO_DIR"));
            }
            home_dir().map(|h| {
                #[cfg(windows)]
                let path = std::path::Path::new(&h).join("AppData").join("Local").join("deno").to_string_lossy().into_owned();
                #[cfg(not(windows))]
                let path = std::path::Path::new(&h).join(".cache").join("deno").to_string_lossy().into_owned();
                (path, "default")
            })
        }),
        list_cmd: Some(&["deno", "info"]),
        list_fn: None,
    }
}

/// `deno --version` → "deno 2.1.1\nv8 ...\ntypescript ..."
/// Extract "2.1.1".
fn deno_version(output: &str) -> Option<String> {
    output.lines()
        .find(|l| l.trim_start().starts_with("deno "))?
        .split_whitespace()
        .nth(1)
        .map(|s| s.to_string())
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
