use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "NuGet",
        command: "nuget",
        category: Category::Language,
        version_flag: "help",
        version_extractor: None,
        config_paths: &[
            "~\\.nuget\\NuGet\\NuGet.Config",
            "%APPDATA%\\NuGet\\NuGet.Config",
        ],
        env_vars: &["NUGET_PACKAGES", "NUGET_HTTP_CACHE_PATH"],
        packages_dir: Some(|env| {
            env.get("NUGET_PACKAGES").cloned().or_else(|| {
                std::env::var("USERPROFILE")
                    .ok()
                    .map(|p| format!("{}\\.nuget\\packages", p))
            })
        }),
        list_cmd: None,
    }
}
