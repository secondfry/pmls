use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "RubyGems",
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
        packages_dir: Some(|env| {
            env.get("GEM_HOME").cloned()
        }),
        list_cmd: Some(&["gem", "list"]),
    }
}
