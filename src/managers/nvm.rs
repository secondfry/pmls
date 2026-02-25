use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Node Version Manager",
        command: "nvm",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: Some(nvm_version),
        config_paths: &[
            "~/.nvmrc",
            ".nvmrc",
        ],
        env_vars: &[
            "NVM_HOME",
            "NVM_SYMLINK",
            "NVM_DIR",
        ],
        packages_dir: Some(|env| {
            env.get("NVM_HOME").cloned()
                .or_else(|| env.get("NVM_DIR").cloned())
                .or_else(|| {
                    std::env::var("HOME").ok().map(|h| {
                        std::path::Path::new(&h).join(".nvm").to_string_lossy().into_owned()
                    })
                })
        }),
        list_cmd: Some(&["nvm", "list"]),
    }
}

/// nvm-windows: "Running version 1.1.9."  Unix nvm: "0.39.7"
fn nvm_version(output: &str) -> Option<String> {
    let first = output.lines().find(|l| !l.trim().is_empty())?.trim();
    if let Some(v) = first.strip_prefix("Running version ") {
        return Some(v.trim_end_matches('.').to_string());
    }
    Some(first.to_string())
}
