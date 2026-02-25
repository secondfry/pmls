use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Apache Maven",
        command: "mvn",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: None,
        config_paths: &[
            "~/.m2/settings.xml",
            "~/.m2/settings-security.xml",
            "pom.xml",
        ],
        env_vars: &[
            "MAVEN_HOME",
            "M2_HOME",
            "MAVEN_OPTS",
            "MAVEN_CONFIG",
        ],
        packages_dir: Some(|_env| {
            home_dir().map(|h| {
                std::path::Path::new(&h).join(".m2").join("repository").to_string_lossy().into_owned()
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
