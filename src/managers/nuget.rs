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
            if let Some(p) = env.get("NUGET_PACKAGES") {
                return Some((p.clone(), "$NUGET_PACKAGES"));
            }
            std::env::var("USERPROFILE")
                .ok()
                .map(|h| (std::path::Path::new(&h).join(".nuget").join("packages").to_string_lossy().into_owned(), "default"))
        }),
        list_cmd: None,
        list_fn: None,
    }
}
