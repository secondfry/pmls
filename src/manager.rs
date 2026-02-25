#[derive(Debug, Clone, PartialEq)]
pub enum Category {
    System,
    Language,
    Universal,
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Category::System => write!(f, "System"),
            Category::Language => write!(f, "Language"),
            Category::Universal => write!(f, "Universal"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PackageManager {
    pub name: &'static str,
    pub command: &'static str,
    pub category: Category,
    /// Flag passed to get version output, e.g. "--version".
    pub version_flag: &'static str,
    /// Optional post-processor applied to raw version output.
    pub version_extractor: Option<fn(&str) -> Option<String>>,
    /// Notable config file / directory paths.
    /// May use `~` for home directory or platform-specific variables as
    /// documentation hints (e.g. `%APPDATA%`, `$XDG_CONFIG_HOME`).
    pub config_paths: &'static [&'static str],
    /// Environment variables that influence the manager's behaviour
    /// (install root, cache dir, registry, proxy, etc.).
    pub env_vars: &'static [&'static str],
    /// Runtime function that resolves the primary directory where packages
    /// or binaries installed by this manager live.
    /// Returns `None` when the location cannot be determined at runtime.
    pub packages_dir: Option<fn() -> Option<String>>,
}

#[derive(Debug)]
pub struct DetectedPackageManager {
    pub manager: PackageManager,
    pub version: Option<String>,
    /// Resolved packages/binaries directory (populated during detection).
    pub packages_dir: Option<String>,
}
