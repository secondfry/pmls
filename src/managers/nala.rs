use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Nala",
        command: "nala",
        category: Category::System,
        version_flag: "--version",
        version_extractor: Some(nala_version),
        config_paths: &[
            "/etc/nala/sources.list",
            "/var/lib/nala/",
        ],
        env_vars: &[],
        packages_dir: Some(|_env| {
            Some(("/var/cache/nala".to_string(), "default"))
        }),
        list_cmd: Some(&["nala", "list", "--installed"]),
        list_fn: None,
    }
}

/// `nala --version` → "nala 0.15.4" — extract "0.15.4".
fn nala_version(output: &str) -> Option<String> {
    output.trim()
        .strip_prefix("nala ")
        .map(|s| s.to_string())
}
