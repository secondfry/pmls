use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "pkg (FreeBSD / Termux)",
        command: "pkg",
        category: Category::System,
        version_flag: "--version",
        version_extractor: None,
        config_paths: &[
            "/usr/local/etc/pkg.conf",
            "/etc/pkg/FreeBSD.conf",
        ],
        env_vars: &[
            "PKG_DBDIR",
            "PKG_CACHEDIR",
            "PACKAGEROOT",
        ],
        packages_dir: Some(|env| {
            if let Some(p) = env.get("PKG_CACHEDIR") {
                return Some((p.clone(), "$PKG_CACHEDIR"));
            }
            Some(("/var/cache/pkg".to_string(), "default"))
        }),
        list_cmd: Some(&["pkg", "info"]),
        list_fn: None,
    }
}
