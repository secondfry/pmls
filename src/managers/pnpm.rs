use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "pnpm",
        command: "pnpm",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: None,
        config_paths: &[
            "~/.npmrc",
            "~/.pnpmfile.cjs",
        ],
        env_vars: &[
            "PNPM_HOME",
            "NPM_CONFIG_REGISTRY",
        ],
        packages_dir: Some(|env| {
            env.get("PNPM_HOME").map(|v| (v.clone(), "$PNPM_HOME"))
        }),
        list_cmd: Some(&["pnpm", "-g", "ls", "--depth=0"]),
        list_fn: None,
    }
}
