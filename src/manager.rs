use std::collections::HashMap;
use serde::Serialize;

/// Resolved values of a manager's declared environment variables.
/// Keys are the static variable names from `PackageManager::env_vars`.
pub type EnvMap = HashMap<&'static str, String>;

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
    pub packages_dir: Option<fn(&EnvMap) -> Option<(String, &'static str)>>,
    /// Command + arguments used to list installed packages, e.g.
    /// `&["npm", "-g", "ls", "--depth=0"]`.
    /// The first element must be the executable; the rest are arguments.
    /// `None` means the manager has no simple list command.
    pub list_cmd: Option<&'static [&'static str]>,
    /// Optional closure for listing when a simple static command is not
    /// sufficient (e.g. glob expansion or runtime path resolution is required).
    /// Takes precedence over `list_cmd` when both are set.
    pub list_fn: Option<fn(&EnvMap) -> Result<Vec<String>, String>>,
}

#[derive(Debug)]
pub struct DetectedPackageManager {
    pub manager: PackageManager,
    pub version: Option<String>,
    /// Resolved packages/binaries directory (populated during detection).
    pub packages_dir: Option<String>,
    /// Where packages_dir came from: env var name, config file path, "default", etc.
    pub packages_dir_source: Option<&'static str>,
}

// ── JSON output types ─────────────────────────────────────────────────────────

/// Serialisable representation of a single detected manager.
/// `PackageManager` itself cannot derive `Serialize` because it contains
/// raw function pointers, so we map into this struct for `--json` output.
#[derive(Serialize)]
pub struct JsonEntry {
    pub command: String,
    pub name: String,
    pub category: String,
    pub version: Option<String>,
    pub packages_dir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packages_dir_source: Option<String>,
    /// Only present when `--list` was requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packages: Option<Vec<String>>,
    /// Only present when `--list` was requested and the command failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_error: Option<String>,
}
