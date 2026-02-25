use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "cpanminus (Perl)",
        command: "cpanm",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: Some(cpanm_version),
        config_paths: &[
            "~/.cpanm/",
            "cpanfile",
        ],
        env_vars: &[
            "PERL_CPANM_HOME",
            "PERL_CPANM_OPT",
            "PERL_MM_OPT",
            "PERL_MB_OPT",
        ],
        packages_dir: Some(|env| {
            if let Some(p) = env.get("PERL_CPANM_HOME") {
                return Some((p.clone(), "$PERL_CPANM_HOME"));
            }
            home_dir().map(|h| {
                (std::path::Path::new(&h).join(".cpanm").to_string_lossy().into_owned(), "default")
            })
        }),
        list_cmd: None,
        list_fn: None,
    }
}

/// `cpanm --version` → "cpanm (App::cpanminus) version 1.7047 ..." — extract "1.7047".
fn cpanm_version(output: &str) -> Option<String> {
    let line = output.lines().find(|l| l.contains("version"))?;
    line.split("version ")
        .nth(1)
        .and_then(|s| s.split_whitespace().next())
        .map(|s| s.to_string())
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
