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
        packages_dir: Some(|env| {
            env.get("GOMODCACHE").cloned().or_else(|| {
                env.get("GOPATH")
                    .map(|p| format!("{}/pkg/mod", p))
            })
        }),
        list_cmd: None,
    }
}
