use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Nix",
        command: "nix",
        category: Category::Universal,
        version_flag: "--version",
        version_extractor: None,
        config_paths: &[
            "/etc/nix/nix.conf",
            "~/.config/nix/nix.conf",
        ],
        env_vars: &["NIX_PATH", "NIX_STORE", "NIX_CONF_DIR"],
        packages_dir: Some(|| {
            std::env::var("NIX_STORE")
                .ok()
                .or_else(|| Some("/nix/store".to_string()))
        }),
        list_cmd: Some(&["nix", "profile", "list"]),
    }
}
