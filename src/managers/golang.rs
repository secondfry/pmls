use std::process::Command;

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
                // GOPATH may be a list of paths separated by os.PathListSeparator.
                // The bin dir is always under the *first* entry.
                let first = p.split(if cfg!(windows) { ';' } else { ':' }).next().unwrap_or(p);
                return Some((std::path::Path::new(first).join("bin").to_string_lossy().into_owned(), "$GOPATH"));
            }
            home_dir().map(|h| {
                (std::path::Path::new(&h).join("go").join("bin").to_string_lossy().into_owned(), "default")
            })
        }),
        list_cmd: None,
        list_fn: Some(|env| {
            // Resolve the binary directory (same priority as packages_dir).
            let bin_dir = if let Some(p) = env.get("GOBIN") {
                p.clone()
            } else if let Some(p) = env.get("GOPATH") {
                // GOPATH may be a list; bin is always under the first entry.
                let first = p.split(if cfg!(windows) { ';' } else { ':' }).next().unwrap_or(p.as_str());
                std::path::Path::new(first).join("bin").to_string_lossy().into_owned()
            } else {
                match home_dir() {
                    Some(h) => std::path::Path::new(&h).join("go").join("bin").to_string_lossy().into_owned(),
                    None => return Ok(vec![]),
                }
            };

            // Collect all files in the bin dir.
            let paths: Vec<std::path::PathBuf> = match std::fs::read_dir(&bin_dir) {
                Ok(e) => e.filter_map(|e| e.ok().map(|e| e.path()))
                    // include regular files and symlinks (gobin-info does the same)
                    .filter(|p| p.is_file() || p.is_symlink())
                    .collect(),
                Err(_) => return Ok(vec![]),
            };

            if paths.is_empty() {
                return Ok(vec![]);
            }

            // Run: go version -m <file1> <file2> ...
            let mut cmd = Command::new("go");
            cmd.arg("version").arg("-m");
            for p in &paths {
                cmd.arg(p);
            }
            let output = cmd.output().map_err(|e| e.to_string())?;
            let text = String::from_utf8_lossy(&output.stdout).into_owned();

            // Parse output:
            //   binary_path: go1.x.y
            //       path    tool/import/path
            //       mod     module/path  vX.Y.Z  h1:...
            let mut results = Vec::new();
            let mut current_name = String::new();
            for line in text.lines() {
                if !line.starts_with('\t') {
                    // "D:\go\bin\pmls.exe: go1.21.5"
                    let path_part = line.split(':').next().unwrap_or("").trim();
                    current_name = std::path::Path::new(path_part)
                        .file_name()
                        .map(|n| n.to_string_lossy().into_owned())
                        .unwrap_or_else(|| path_part.to_string());
                } else {
                    // "\tmod\tgithub.com/foo/bar\tv1.2.3\th1:..."
                    let parts: Vec<&str> = line.trim_start_matches('\t').splitn(4, '\t').collect();
                    if parts.first() == Some(&"mod") && parts.len() >= 3 {
                        results.push(format!("{}  {}  {}", current_name, parts[1], parts[2]));
                    }
                }
            }
            Ok(results)
        }),
    }
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
