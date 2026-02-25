use std::process::Command;

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
    /// Flag passed to get version output, e.g. "--version"
    pub version_flag: &'static str,
}

#[derive(Debug)]
pub struct DetectedPackageManager {
    pub manager: PackageManager,
    pub version: Option<String>,
}

/// All known package managers. Detection is purely command-based, so this
/// works cross-platform: entries unavailable on the current OS simply won't
/// be found.
fn all_package_managers() -> Vec<PackageManager> {
    vec![
        // ── Windows ──────────────────────────────────────────────────────────
        PackageManager { name: "Chocolatey",    command: "choco",    category: Category::System,   version_flag: "--version" },
        PackageManager { name: "Scoop",         command: "scoop",    category: Category::System,   version_flag: "--version" },
        PackageManager { name: "Winget",        command: "winget",   category: Category::System,   version_flag: "--version" },
        PackageManager { name: "NuGet",         command: "nuget",    category: Category::Language, version_flag: "help" },
        // ── Linux / macOS ────────────────────────────────────────────────────
        PackageManager { name: "APT",           command: "apt",      category: Category::System,   version_flag: "--version" },
        PackageManager { name: "APT-GET",       command: "apt-get",  category: Category::System,   version_flag: "--version" },
        PackageManager { name: "Pacman",        command: "pacman",   category: Category::System,   version_flag: "--version" },
        PackageManager { name: "DNF",           command: "dnf",      category: Category::System,   version_flag: "--version" },
        PackageManager { name: "YUM",           command: "yum",      category: Category::System,   version_flag: "--version" },
        PackageManager { name: "Zypper",        command: "zypper",   category: Category::System,   version_flag: "--version" },
        PackageManager { name: "APK",           command: "apk",      category: Category::System,   version_flag: "--version" },
        PackageManager { name: "Snap",          command: "snap",     category: Category::Universal, version_flag: "--version" },
        PackageManager { name: "Flatpak",       command: "flatpak",  category: Category::Universal, version_flag: "--version" },
        PackageManager { name: "Homebrew",      command: "brew",     category: Category::System,   version_flag: "--version" },
        PackageManager { name: "MacPorts",      command: "port",     category: Category::System,   version_flag: "version" },
        PackageManager { name: "Nix",           command: "nix",      category: Category::Universal, version_flag: "--version" },
        PackageManager { name: "XBPS",          command: "xbps-install", category: Category::System, version_flag: "--version" },
        PackageManager { name: "Emerge (Portage)", command: "emerge", category: Category::System,  version_flag: "--version" },
        PackageManager { name: "Eopkg",         command: "eopkg",    category: Category::System,   version_flag: "--version" },
        // ── Language / universal ─────────────────────────────────────────────
        PackageManager { name: "Cargo (Rust)",  command: "cargo",    category: Category::Language, version_flag: "--version" },
        PackageManager { name: "npm (Node.js)", command: "npm",      category: Category::Language, version_flag: "--version" },
        PackageManager { name: "Yarn",          command: "yarn",     category: Category::Language, version_flag: "--version" },
        PackageManager { name: "pnpm",          command: "pnpm",     category: Category::Language, version_flag: "--version" },
        PackageManager { name: "Bun",           command: "bun",      category: Category::Language, version_flag: "--version" },
        PackageManager { name: "pip (Python)",  command: "pip",      category: Category::Language, version_flag: "--version" },
        PackageManager { name: "pip3 (Python)", command: "pip3",     category: Category::Language, version_flag: "--version" },
        PackageManager { name: "uv (Python)",   command: "uv",       category: Category::Language, version_flag: "--version" },
        PackageManager { name: "Gem (Ruby)",    command: "gem",      category: Category::Language, version_flag: "--version" },
        PackageManager { name: "Bundler",       command: "bundle",   category: Category::Language, version_flag: "--version" },
        PackageManager { name: "Composer (PHP)", command: "composer", category: Category::Language, version_flag: "--version" },
        PackageManager { name: "Go modules",    command: "go",       category: Category::Language, version_flag: "version" },
        PackageManager { name: "Maven",         command: "mvn",      category: Category::Language, version_flag: "--version" },
        PackageManager { name: "Gradle",        command: "gradle",   category: Category::Language, version_flag: "--version" },
        PackageManager { name: "dotnet CLI",    command: "dotnet",   category: Category::Language, version_flag: "--version" },
        PackageManager { name: "Conda",         command: "conda",    category: Category::Language, version_flag: "--version" },
        PackageManager { name: "Helm (K8s)",    command: "helm",     category: Category::Universal, version_flag: "version" },
    ]
}

/// Returns `true` if the given command is reachable on `PATH`.
fn command_exists(cmd: &str) -> bool {
    #[cfg(windows)]
    let result = Command::new("where").arg(cmd).output();
    #[cfg(not(windows))]
    let result = Command::new("which").arg(cmd).output();

    result.map(|o| o.status.success()).unwrap_or(false)
}

/// Runs `cmd <flag>` and returns the first non-empty line of output.
fn get_version(cmd: &str, flag: &str) -> Option<String> {
    let output = Command::new(cmd).arg(flag).output().ok()?;
    // Some tools print version to stderr (e.g. certain Python builds)
    let raw = if output.stdout.is_empty() {
        output.stderr
    } else {
        output.stdout
    };
    let text = String::from_utf8_lossy(&raw);
    text.lines()
        .find(|l| !l.trim().is_empty())
        .map(|l| l.trim().to_string())
}

fn detect() -> Vec<DetectedPackageManager> {
    all_package_managers()
        .into_iter()
        .filter_map(|pm| {
            if command_exists(pm.command) {
                let version = get_version(pm.command, pm.version_flag);
                Some(DetectedPackageManager { manager: pm, version })
            } else {
                None
            }
        })
        .collect()
}

fn main() {
    let detected = detect();

    if detected.is_empty() {
        println!("No package managers detected.");
        return;
    }

    println!("Detected {} package manager(s):\n", detected.len());

    // Group by category for readable output
    for category in [Category::System, Category::Language, Category::Universal] {
        let group: Vec<_> = detected
            .iter()
            .filter(|d| d.manager.category == category)
            .collect();

        if group.is_empty() {
            continue;
        }

        println!("  [{}]", category);
        for d in &group {
            match &d.version {
                Some(v) => println!("    ✓ {} ({})", d.manager.name, v),
                None    => println!("    ✓ {} (version unknown)", d.manager.name),
            }
        }
        println!();
    }
}

