use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Go modules",
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
            "GOPATH",
            "GOROOT",
            "GOMODCACHE",
            "GOPROXY",
            "GONOSUMDB",
            "GOFLAGS",
        ],
        packages_dir: Some(|| {
            std::env::var("GOMODCACHE").ok().or_else(|| {
                std::env::var("GOPATH")
                    .ok()
                    .map(|p| format!("{}/pkg/mod", p))
            })
        }),
    }
}
