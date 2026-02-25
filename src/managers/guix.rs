use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "GNU Guix",
        command: "guix",
        category: Category::System,
        version_flag: "--version",
        version_extractor: Some(guix_version),
        config_paths: &[
            "~/.config/guix/",
            "/etc/guix/channels.scm",
        ],
        env_vars: &[
            "GUIX_PROFILE",
            "GUIX_PACKAGE_PATH",
            "GUIX_LOCPATH",
        ],
        packages_dir: Some(|env| {
            if let Some(p) = env.get("GUIX_PROFILE") {
                return Some((p.clone(), "$GUIX_PROFILE"));
            }
            home_dir().map(|h| {
                (std::path::Path::new(&h).join(".guix-profile").to_string_lossy().into_owned(), "default")
            })
        }),
        list_cmd: Some(&["guix", "package", "--list-installed"]),
        list_fn: None,
    }
}

/// `guix --version` → "guix (GNU Guix) 1.4.0\n..." — extract "1.4.0".
fn guix_version(output: &str) -> Option<String> {
    output.lines()
        .next()?
        .split(')')
        .nth(1)?
        .split_whitespace()
        .next()
        .map(|s| s.to_string())
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
