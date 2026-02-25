use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: ".NET CLI",
        command: "dotnet",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: None,
        config_paths: &[
            "~/.nuget/NuGet/NuGet.Config",
            "%APPDATA%\\NuGet\\NuGet.Config",
            "nuget.config",
        ],
        env_vars: &[
            "DOTNET_ROOT",
            "DOTNET_CLI_HOME",
            "DOTNET_NOLOGO",
            "NUGET_PACKAGES",
            "NUGET_HTTP_CACHE_PATH",
        ],
        packages_dir: Some(|| {
            std::env::var("NUGET_PACKAGES").ok().or_else(|| {
                home_dir().map(|h| format!("{}/.nuget/packages", h))
            })
        }),
        list_cmd: Some(&["dotnet", "tool", "list", "-g"]),
    }
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
