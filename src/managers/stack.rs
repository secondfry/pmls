use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Stack (Haskell)",
        command: "stack",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: Some(stack_version),
        config_paths: &[
            "~/.stack/config.yaml",
            "stack.yaml",
            "stack.yaml.lock",
        ],
        env_vars: &[
            "STACK_ROOT",
            "STACK_YAML",
            "STACK_WORK",
        ],
        packages_dir: Some(|env| {
            if let Some(p) = env.get("STACK_ROOT") {
                return Some((p.clone(), "$STACK_ROOT"));
            }
            home_dir().map(|h| {
                (std::path::Path::new(&h).join(".stack").to_string_lossy().into_owned(), "default")
            })
        }),
        list_cmd: Some(&["stack", "ls", "dependencies"]),
        list_fn: None,
    }
}

/// `stack --version` → "Version 3.1.1, Git revision ..." — extract "3.1.1".
fn stack_version(output: &str) -> Option<String> {
    output.trim()
        .strip_prefix("Version ")
        .and_then(|s| s.split(',').next())
        .map(|s| s.to_string())
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
