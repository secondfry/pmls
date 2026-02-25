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
    /// Optional post-processor applied to the raw version output.
    /// Receives the full stdout/stderr text, returns the display version string.
    pub version_extractor: Option<fn(&str) -> Option<String>>,
}

#[derive(Debug)]
pub struct DetectedPackageManager {
    pub manager: PackageManager,
    pub version: Option<String>,
}

/// Extracts the version from `scoop --version` output.
/// Handles two formats:
///   interactive:    "b588a06e (HEAD -> master, tag: v0.5.3, ...) ..."  → "v0.5.3"
///   non-interactive "b588a06e chore(release): Bump to version 0.5.3"   → "v0.5.3"
fn scoop_version(output: &str) -> Option<String> {
    for line in output.lines() {
        // Case 1: git log decoration present, e.g. "tag: v0.5.3"
        if let Some(pos) = line.find("tag: ") {
            let after = &line[pos + 5..];
            let ver: String = after
                .chars()
                .take_while(|c| !matches!(c, ',' | ')' | ' '))
                .collect();
            if !ver.is_empty() {
                return Some(ver);
            }
        }
        // Case 2: commit message contains "version X.Y.Z"
        if let Some(pos) = line.to_lowercase().find("version ") {
            let after = &line[pos + 8..];
            let ver: String = after
                .chars()
                .take_while(|c| c.is_ascii_digit() || *c == '.')
                .collect();
            if ver.contains('.') {
                return Some(format!("v{}", ver));
            }
        }
    }
    // Fallback: second non-empty line as-is
    output
        .lines()
        .filter(|l| !l.trim().is_empty())
        .nth(1)
        .map(|l| l.trim().to_string())
}

/// All known package managers. Detection is purely command-based, so this
/// works cross-platform: entries unavailable on the current OS simply won't
/// be found.
fn all_package_managers() -> Vec<PackageManager> {
    vec![
        // ── Windows ──────────────────────────────────────────────────────────
        PackageManager { name: "Chocolatey",    command: "choco",    category: Category::System,   version_flag: "--version", version_extractor: None },
        PackageManager { name: "Scoop",         command: "scoop",    category: Category::System,   version_flag: "--version", version_extractor: Some(scoop_version) },
        PackageManager { name: "Winget",        command: "winget",   category: Category::System,   version_flag: "--version", version_extractor: None },
        PackageManager { name: "NuGet",         command: "nuget",    category: Category::Language, version_flag: "help",      version_extractor: None },
        // ── Linux / macOS ────────────────────────────────────────────────────
        PackageManager { name: "APT",           command: "apt",      category: Category::System,   version_flag: "--version", version_extractor: None },
        PackageManager { name: "APT-GET",       command: "apt-get",  category: Category::System,   version_flag: "--version", version_extractor: None },
        PackageManager { name: "Pacman",        command: "pacman",   category: Category::System,   version_flag: "--version", version_extractor: None },
        PackageManager { name: "DNF",           command: "dnf",      category: Category::System,   version_flag: "--version", version_extractor: None },
        PackageManager { name: "YUM",           command: "yum",      category: Category::System,   version_flag: "--version", version_extractor: None },
        PackageManager { name: "Zypper",        command: "zypper",   category: Category::System,   version_flag: "--version", version_extractor: None },
        PackageManager { name: "APK",           command: "apk",      category: Category::System,   version_flag: "--version", version_extractor: None },
        PackageManager { name: "Snap",          command: "snap",     category: Category::Universal, version_flag: "--version", version_extractor: None },
        PackageManager { name: "Flatpak",       command: "flatpak",  category: Category::Universal, version_flag: "--version", version_extractor: None },
        PackageManager { name: "Homebrew",      command: "brew",     category: Category::System,   version_flag: "--version", version_extractor: None },
        PackageManager { name: "MacPorts",      command: "port",     category: Category::System,   version_flag: "version",   version_extractor: None },
        PackageManager { name: "Nix",           command: "nix",      category: Category::Universal, version_flag: "--version", version_extractor: None },
        PackageManager { name: "XBPS",          command: "xbps-install", category: Category::System, version_flag: "--version", version_extractor: None },
        PackageManager { name: "Emerge (Portage)", command: "emerge", category: Category::System,  version_flag: "--version", version_extractor: None },
        PackageManager { name: "Eopkg",         command: "eopkg",    category: Category::System,   version_flag: "--version", version_extractor: None },
        // ── Language / universal ─────────────────────────────────────────────
        PackageManager { name: "Cargo (Rust)",  command: "cargo",    category: Category::Language, version_flag: "--version", version_extractor: None },
        PackageManager { name: "npm (Node.js)", command: "npm",      category: Category::Language, version_flag: "--version", version_extractor: None },
        PackageManager { name: "Yarn",          command: "yarn",     category: Category::Language, version_flag: "--version", version_extractor: None },
        PackageManager { name: "pnpm",          command: "pnpm",     category: Category::Language, version_flag: "--version", version_extractor: None },
        PackageManager { name: "Bun",           command: "bun",      category: Category::Language, version_flag: "--version", version_extractor: None },
        PackageManager { name: "pip (Python)",  command: "pip",      category: Category::Language, version_flag: "--version", version_extractor: None },
        PackageManager { name: "pip3 (Python)", command: "pip3",     category: Category::Language, version_flag: "--version", version_extractor: None },
        PackageManager { name: "uv (Python)",   command: "uv",       category: Category::Language, version_flag: "--version", version_extractor: None },
        PackageManager { name: "Gem (Ruby)",    command: "gem",      category: Category::Language, version_flag: "--version", version_extractor: None },
        PackageManager { name: "Bundler",       command: "bundle",   category: Category::Language, version_flag: "--version", version_extractor: None },
        PackageManager { name: "Composer (PHP)", command: "composer", category: Category::Language, version_flag: "--version", version_extractor: None },
        PackageManager { name: "Go modules",    command: "go",       category: Category::Language, version_flag: "version",   version_extractor: None },
        PackageManager { name: "Maven",         command: "mvn",      category: Category::Language, version_flag: "--version", version_extractor: None },
        PackageManager { name: "Gradle",        command: "gradle",   category: Category::Language, version_flag: "--version", version_extractor: None },
        PackageManager { name: "dotnet CLI",    command: "dotnet",   category: Category::Language, version_flag: "--version", version_extractor: None },
        PackageManager { name: "Conda",         command: "conda",    category: Category::Language, version_flag: "--version", version_extractor: None },
        PackageManager { name: "Helm (K8s)",    command: "helm",     category: Category::Universal, version_flag: "version",  version_extractor: None },
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

/// Runs `cmd <flag>` and returns a version string.
/// On Windows all commands are dispatched through `cmd /C` so that `.cmd`/`.bat`
/// wrappers (e.g. npm, yarn, pnpm) are resolved correctly.
fn get_version(cmd: &str, flag: &str, extractor: Option<fn(&str) -> Option<String>>) -> Option<String> {
    #[cfg(windows)]
    let output = Command::new("cmd").args(["/C", cmd, flag]).output().ok()?;
    #[cfg(not(windows))]
    let output = Command::new(cmd).arg(flag).output().ok()?;

    // Some tools print version to stderr (e.g. certain Python builds)
    let raw = if output.stdout.is_empty() {
        output.stderr
    } else {
        output.stdout
    };
    let text = String::from_utf8_lossy(&raw);
    let text = text.as_ref();

    if let Some(f) = extractor {
        return f(text);
    }

    text.lines()
        .find(|l| !l.trim().is_empty())
        .map(|l| l.trim().to_string())
}

fn detect() -> Vec<DetectedPackageManager> {
    all_package_managers()
        .into_iter()
        .filter_map(|pm| {
            if command_exists(pm.command) {
                let version = get_version(pm.command, pm.version_flag, pm.version_extractor);
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

