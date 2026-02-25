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
        packages_dir: Some(|env| {
            if let Some(p) = env.get("NIX_STORE") {
                return Some((p.clone(), "$NIX_STORE"));
            }
            Some(("/nix/store".to_string(), "default"))
        }),
        list_cmd: Some(&["nix", "profile", "list"]),
        list_fn: None,
    }
}
