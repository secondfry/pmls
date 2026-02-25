use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "package installer for Python",
        command: "pip3",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: Some(pip3_version),
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
            let out = std::process::Command::new("python3")
                .args(["-c", "import sysconfig; print(sysconfig.get_path('purelib'))"])
                .output()
                .ok()?;
            let text = String::from_utf8_lossy(&out.stdout);
            let path = text.trim();
            if path.is_empty() { None } else { Some(path.to_string()) }
        }),
        list_cmd: Some(&["pip3", "list"]),
    }
}

fn pip3_version(output: &str) -> Option<String> {
    output.trim()
        .strip_prefix("pip ")
        .and_then(|s| s.split_whitespace().next())
        .map(|s| s.to_string())
}


