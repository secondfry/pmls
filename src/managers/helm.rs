use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Helm",
        command: "helm",
        category: Category::Universal,
        version_flag: "version",
        version_extractor: None,
        config_paths: &[
            "~/.config/helm/",
            "~/.helm/",
        ],
        env_vars: &[
            "HELM_HOME",
            "HELM_CACHE_HOME",
            "HELM_CONFIG_HOME",
            "HELM_DATA_HOME",
            "KUBECONFIG",
        ],
        packages_dir: Some(|env| {
            if let Some(p) = env.get("HELM_DATA_HOME") {
                return Some((p.clone(), "$HELM_DATA_HOME"));
            }
            home_dir().map(|h| {
                #[cfg(windows)]
                return (format!("{}\\AppData\\Roaming\\helm", h), "default");
                #[cfg(not(windows))]
                return (format!("{}/.local/share/helm", h), "default");
            })
        }),
        list_cmd: Some(&["helm", "list", "-A"]),
    }
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
