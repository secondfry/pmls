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
            if let Some(p) = env.get("GOMODCACHE") {
                return Some((p.clone(), "$GOMODCACHE"));
            }
            env.get("GOPATH")
                .map(|p| (std::path::Path::new(p).join("pkg").join("mod").to_string_lossy().into_owned(), "$GOPATH"))
        }),
        list_cmd: None,
    }
}
