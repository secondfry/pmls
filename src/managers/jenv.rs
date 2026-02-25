use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "jenv",
        command: "jenv",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: Some(jenv_version),
        config_paths: &[
            "~/.jenv/",
            ".java-version",
        ],
        env_vars: &[
            "JENV_ROOT",
            "JAVA_HOME",
        ],
        packages_dir: Some(|env| {
            if let Some(p) = env.get("JENV_ROOT") {
                return Some((
                    std::path::Path::new(p).join("versions").to_string_lossy().into_owned(),
                    "$JENV_ROOT/versions",
                ));
            }
            home_dir().map(|h| {
                (std::path::Path::new(&h).join(".jenv").join("versions").to_string_lossy().into_owned(), "default")
            })
        }),
        list_cmd: Some(&["jenv", "versions"]),
        list_fn: None,
    }
}

/// `jenv --version` → "jenv 0.5.6" — extract "0.5.6".
fn jenv_version(output: &str) -> Option<String> {
    output.trim()
        .strip_prefix("jenv ")
        .map(|s| s.to_string())
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
