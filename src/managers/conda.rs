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
            env.get("CONDA_PKGS_DIRS").cloned().or_else(|| {
                env.get("CONDA_PREFIX")
                    .map(|p| format!("{}/pkgs", p))
            })
        }),
        list_cmd: Some(&["conda", "list"]),
    }
}
