use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "LuaRocks",
        command: "luarocks",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: Some(luarocks_version),
        config_paths: &[
            "~/.luarocks/",
            ".luarocks/",
        ],
        env_vars: &[
            "LUAROCKS_CONFIG",
            "LUA_PATH",
            "LUA_CPATH",
        ],
        packages_dir: Some(|_env| {
            home_dir().map(|h| {
                (std::path::Path::new(&h).join(".luarocks").to_string_lossy().into_owned(), "default")
            })
        }),
        list_cmd: Some(&["luarocks", "list"]),
        list_fn: None,
    }
}

/// `luarocks --version` → "LuaRocks 3.11.1\n..." — extract "3.11.1".
fn luarocks_version(output: &str) -> Option<String> {
    output.lines()
        .next()?
        .strip_prefix("LuaRocks ")
        .map(|s| s.to_string())
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
