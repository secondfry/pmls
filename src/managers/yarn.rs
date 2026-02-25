use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Yarn",
        command: "yarn",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: None,
        config_paths: &[
            "~/.yarnrc",
            "~/.yarnrc.yml",
        ],
        env_vars: &[
            "YARN_CACHE_FOLDER",
            "YARN_GLOBAL_FOLDER",
            "YARN_REGISTRY",
        ],
        packages_dir: Some(|env| {
            env.get("YARN_GLOBAL_FOLDER").map(|v| (v.clone(), "$YARN_GLOBAL_FOLDER"))
        }),
        list_cmd: Some(&["yarn", "global", "list", "--depth=0"]),
    }
}
