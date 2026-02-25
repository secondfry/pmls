use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Bundler",
        command: "bundle",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: None,
        config_paths: &[
            "~/.bundle/config",
            ".bundle/config",
            "Gemfile",
        ],
        env_vars: &[
            "BUNDLE_PATH",
            "BUNDLE_APP_CONFIG",
            "BUNDLE_GEMFILE",
            "BUNDLE_WITHOUT",
        ],
        packages_dir: Some(|| {
            std::env::var("BUNDLE_PATH").ok()
        }),
        list_cmd: Some(&["bundle", "list"]),
    }
}
