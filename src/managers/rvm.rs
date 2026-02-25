use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Ruby Version Manager",
        command: "rvm",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: Some(rvm_version),
        config_paths: &[
            "~/.rvm/",
            "~/.rvmrc",
            ".rvmrc",
        ],
        env_vars: &[
            "rvm_path",
            "GEM_HOME",
            "MY_RUBY_HOME",
        ],
        packages_dir: Some(|env| {
            if let Some(p) = env.get("rvm_path") {
                return Some((
                    std::path::Path::new(p).join("rubies").to_string_lossy().into_owned(),
                    "$rvm_path/rubies",
                ));
            }
            home_dir().map(|h| {
                (std::path::Path::new(&h).join(".rvm").join("rubies").to_string_lossy().into_owned(), "default")
            })
        }),
        list_cmd: Some(&["rvm", "list"]),
        list_fn: None,
    }
}

/// `rvm --version` → "rvm 1.29.12 (latest) by ..." — extract "1.29.12".
fn rvm_version(output: &str) -> Option<String> {
    let line = output.lines().find(|l| l.contains("rvm "))?;
    line.split_whitespace()
        .skip_while(|w| *w != "rvm")
        .nth(1)
        .map(|s| s.to_string())
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
