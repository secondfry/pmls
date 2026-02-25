use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Conda",
        command: "conda",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: None,
        config_paths: &[
            "~/.condarc",
            "~/conda/.condarc",
        ],
        env_vars: &[
            "CONDA_PREFIX",
            "CONDA_DEFAULT_ENV",
            "CONDA_ENVS_PATH",
            "CONDA_PKGS_DIRS",
        ],
        packages_dir: Some(|env| {
            if let Some(p) = env.get("CONDA_PKGS_DIRS") {
                return Some((p.clone(), "$CONDA_PKGS_DIRS"));
            }
            env.get("CONDA_PREFIX")
                .map(|p| (format!("{}/pkgs", p), "$CONDA_PREFIX"))
        }),
        list_cmd: Some(&["conda", "list"]),
        list_fn: None,
    }
}
