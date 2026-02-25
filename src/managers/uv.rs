use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "uv",
        command: "uv",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: None,
        config_paths: &[
            "~/.config/uv/uv.toml",
            "uv.toml",
        ],
        env_vars: &[
            "UV_CACHE_DIR",
            "UV_PYTHON",
            "UV_TOOL_DIR",
            "UV_PROJECT_ENVIRONMENT",
        ],
        packages_dir: Some(|| {
            std::env::var("UV_TOOL_DIR").ok()
        }),
        list_cmd: Some(&["uv", "tool", "list"]),
    }
}
