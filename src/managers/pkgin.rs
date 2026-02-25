use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "pkgin",
        command: "pkgin",
        category: Category::System,
        version_flag: "--version",
        version_extractor: Some(pkgin_version),
        config_paths: &[
            "/usr/pkg/etc/pkgin/",
            "/etc/pkgin/repositories.conf",
        ],
        env_vars: &[
            "PKG_REPOS",
        ],
        packages_dir: Some(|_env| {
            Some(("/var/db/pkg".to_string(), "default"))
        }),
        list_cmd: Some(&["pkgin", "list"]),
        list_fn: None,
    }
}

/// `pkgin --version` → "pkgin 23.8.1 for ..." — extract "23.8.1".
fn pkgin_version(output: &str) -> Option<String> {
    output.trim()
        .strip_prefix("pkgin ")
        .and_then(|s| s.split_whitespace().next())
        .map(|s| s.to_string())
}
