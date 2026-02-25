use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Gem (Ruby)",
        command: "gem",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: None,
        config_paths: &[
            "~/.gemrc",
        ],
        env_vars: &[
            "GEM_HOME",
            "GEM_PATH",
            "BUNDLE_GEMFILE",
        ],
        packages_dir: Some(|| {
            std::env::var("GEM_HOME").ok()
        }),
        list_cmd: Some(&["gem", "list"]),
    }
}
