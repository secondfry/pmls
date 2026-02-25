use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Go tool installer",
        command: "go",
        category: Category::Language,
        version_flag: "version",
        version_extractor: None,
        config_paths: &[
            "go.mod",
            "go.sum",
            "~/.config/go/env",
        ],
        env_vars: &[
            "GOBIN",
            "GOPATH",
            "GOROOT",
            "GOMODCACHE",
            "GOPROXY",
            "GONOSUMDB",
            "GOFLAGS",
        ],
        packages_dir: Some(|env| {
            if let Some(p) = env.get("GOBIN") {
                return Some((p.clone(), "$GOBIN"));
            }
            if let Some(p) = env.get("GOPATH") {
                return Some((std::path::Path::new(p).join("bin").to_string_lossy().into_owned(), "$GOPATH"));
            }
            home_dir().map(|h| {
                (std::path::Path::new(&h).join("go").join("bin").to_string_lossy().into_owned(), "default")
            })
        }),
        list_cmd: None,
    }
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
