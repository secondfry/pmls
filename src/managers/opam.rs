use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "opam (OCaml)",
        command: "opam",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: None,
        config_paths: &[
            "~/.opam/",
            ".opam-switch/",
            "opam",
            "*.opam",
        ],
        env_vars: &[
            "OPAMROOT",
            "OPAMSWITCH",
            "OPAM_SWITCH_PREFIX",
        ],
        packages_dir: Some(|env| {
            if let Some(p) = env.get("OPAMROOT") {
                return Some((p.clone(), "$OPAMROOT"));
            }
            home_dir().map(|h| {
                (std::path::Path::new(&h).join(".opam").to_string_lossy().into_owned(), "default")
            })
        }),
        list_cmd: Some(&["opam", "list"]),
        list_fn: None,
    }
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
