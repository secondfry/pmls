use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Conan",
        command: "conan",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: Some(conan_version),
        config_paths: &[
            "~/.conan2/",
            "conanfile.txt",
            "conanfile.py",
            "conan_provider.cmake",
        ],
        env_vars: &[
            "CONAN_HOME",
            "CONAN_USER_HOME",
            "CONAN_DEFAULT_PROFILE",
        ],
        packages_dir: Some(|env| {
            if let Some(p) = env.get("CONAN_HOME") {
                return Some((p.clone(), "$CONAN_HOME"));
            }
            home_dir().map(|h| {
                (std::path::Path::new(&h).join(".conan2").to_string_lossy().into_owned(), "default")
            })
        }),
        list_cmd: Some(&["conan", "list"]),
        list_fn: None,
    }
}

/// `conan --version` → "Conan version 2.9.2" — extract "2.9.2".
fn conan_version(output: &str) -> Option<String> {
    output.split_whitespace()
        .last()
        .map(|s| s.to_string())
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
