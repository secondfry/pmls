use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "package installer for Python",
        command: "pip",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: Some(pip_version),
        config_paths: &[
            "~/.pip/pip.conf",
            "~/.config/pip/pip.conf",
            "%APPDATA%\\pip\\pip.ini",
        ],
        env_vars: &[
            "PIP_INDEX_URL",
            "PIP_EXTRA_INDEX_URL",
            "PIP_CACHE_DIR",
            "PIP_CONFIG_FILE",
            "VIRTUAL_ENV",
        ],
        packages_dir: Some(|_env| {
            #[cfg(windows)]
            let out = std::process::Command::new("cmd")
                .args(["/C", "python", "-c", "import sysconfig; print(sysconfig.get_path('purelib'))"])
                .output()
                .ok()?;
            #[cfg(not(windows))]
            let out = std::process::Command::new("python")
                .args(["-c", "import sysconfig; print(sysconfig.get_path('purelib'))"])
                .output()
                .ok()?;
            let text = String::from_utf8_lossy(&out.stdout);
            let path = text.trim();
            if path.is_empty() { None } else { Some((path.to_string(), "python sysconfig")) }
        }),
        list_cmd: Some(&["pip", "list"]),
        list_fn: None,
    }
}

/// `pip --version` → "pip 23.3.1 from ..." — extract just "23.3.1".
fn pip_version(output: &str) -> Option<String> {
    output.trim()
        .strip_prefix("pip ")
        .and_then(|s| s.split_whitespace().next())
        .map(|s| s.to_string())
}


