# pmls

A fast, cross-platform CLI tool written in Rust that lists which package managers are installed on the current machine, reports their version and global packages directory (with provenance), and can enumerate what each has installed.

## Installation

### Via [bin](https://github.com/marcosnils/bin)

```sh
bin install github.com/secondfry/pmls
```

`bin` resolves the latest GitHub release for your platform and installs the binary into your configured `bin` directory.

### Pre-built binaries

Download the binary for your platform from the [releases page](https://github.com/secondfry/pmls/releases) and place it somewhere on your `PATH`.

| Platform | Asset name |
|---|---|
| Windows x86-64 | `pmls-x86_64-pc-windows-msvc.exe` |
| Linux x86-64 | `pmls-x86_64-unknown-linux-gnu` |
| macOS x86-64 | `pmls-x86_64-apple-darwin` |
| macOS arm64 | `pmls-aarch64-apple-darwin` |

### From source

Requires a stable Rust toolchain (edition 2024).

```sh
cargo install --git https://github.com/secondfry/pmls
```

## Usage

```
pmls [OPTIONS]
```

| Flag | Short | Description |
|---|---|---|
| `--list` | `-l` | Run each manager's list command and print installed packages |
| `--json` | `-j` | Output results as a JSON array |
| `--verbose` | `-v` | Show errors when package listing fails |

Flags can be combined freely, e.g. `-lj` produces JSON that includes the installed package list.

### Default example output

```
# System
bin # bin # 0.23.1 # D:\Development\bin (~/.config/bin/config.json)
scoop # Scoop # v0.5.3 # C:\Users\you\scoop\apps (default)
winget # Windows Package Manager # v1.12.460 # C:\Users\you\AppData\Local\Microsoft\WinGet\Packages ($LOCALAPPDATA)

# JavaScript
bun # Bun # 1.3.0 # C:\Users\you\.bun (default)
npm # Node Package Manager # 10.9.0 # C:\Users\you\AppData\Roaming\npm\node_modules (default)
nvm # Node Version Manager # 1.1.9 # D:\Development\nvm-noinstall ($NVM_HOME)

# Python
pip # package installer for Python # 23.3.1 # D:\Program Files\Python310\Lib\site-packages (python sysconfig)
pip3 # package installer for Python # 23.3.1 # D:\Program Files\Python310\Lib\site-packages (python sysconfig)
pipenv # Python virtualenv management tool # 2023.11.15 # C:\Users\you\.virtualenvs (default)

# .NET
dotnet # .NET CLI # 8.0.415 # C:\Users\you\.nuget\packages (default)

# Rust
cargo # Rust package manager # cargo 1.91.1 (ea2d97820 2025-10-10) # D:\Related\rust\cargo ($CARGO_HOME)

# Universal
helm # Helm # version.BuildInfo{Version:"v3.19.0", GitCommit:"3d8990f0836691f0229297773f3524598f46bda6", GitTreeState:"clean", GoVersion:"go1.24.7"} # C:\Users\you\AppData\Roaming\helm (default)
```

Each line follows `command # name # version # packages_dir (source)`. The source in parentheses describes where the path came from:

| Source | Meaning |
|---|---|
| `$VAR_NAME` | Value of a specific environment variable |
| `python sysconfig` | Result of `sysconfig.get_path('purelib')` |
| `~/.config/bin/config.json` | Value read from a config file |
| `default` | Hardcoded OS-appropriate fallback |

The packages directory (and its source) are omitted when the manager has no fixed global location.

### Example output of `--list` mode

```
# System
scoop # Scoop # v0.5.3 # C:\Users\you\scoop\apps (default)
  There aren't any apps installed.
bin # bin # 0.23.1 # D:\Development\bin (~/.config/bin/config.json)
  Path                             Version  URL                                Status
  D:\Development\bin\bin.exe       v0.23.1  github.com/marcosnils/bin          OK
  D:\Development\bin\doggo.exe     v1.1.2   https://github.com/mr-karan/doggo  OK
```

### Example output of `--json` mode

```json
[
  {
    "command": "scoop",
    "name": "Scoop",
    "category": "System",
    "version": "v0.5.3",
    "packages_dir": "C:\\Users\\you\\scoop\\apps",
    "packages_dir_source": "default"
  },
  {
    "command": "cargo",
    "name": "Rust package manager",
    "category": "Language",
    "version": "1.85.0",
    "packages_dir": "D:\\rust\\cargo",
    "packages_dir_source": "$CARGO_HOME"
  }
]
```

With `--list --json` each entry gains a `"packages"` array (and `"list_error"` when `--verbose` is set and listing failed).

## Development build

```sh
cargo build
# binary at target/release/pmls  (pmls.exe on Windows)
```

## Supported package managers

### System — Windows

| Manager | Command | List command |
|---|---|---|
| Chocolatey | `choco` | `choco list --local-only` |
| Scoop | `scoop` | `scoop list` |
| winget | `winget` | `winget list --disable-interactivity` |
| NuGet CLI | `nuget` | — |

### System — Linux

| Manager | Command | List command |
|---|---|---|
| apt | `apt` | `apt list --installed` |
| apt-get | `apt-get` | `apt list --installed` |
| pacman | `pacman` | `pacman -Q` |
| dnf | `dnf` | `dnf list installed` |
| yum | `yum` | `yum list installed` |
| zypper | `zypper` | `zypper packages --installed-only` |
| apk | `apk` | `apk list --installed` |
| snap | `snap` | `snap list` |
| Flatpak | `flatpak` | `flatpak list` |
| Portage | `emerge` | — |
| eopkg | `eopkg` | `eopkg list-installed` |
| xbps | `xbps-query` | `xbps-query -l` |

### System — macOS

| Manager | Command | List command |
|---|---|---|
| Homebrew | `brew` | `brew list` |
| MacPorts | `port` | `port installed` |

### System — cross-platform

| Manager | Command | List command |
|---|---|---|
| bin | `bin` | `bin ls` |

### Universal

| Manager | Command | List command |
|---|---|---|
| Nix | `nix` | `nix profile list` |
| Helm | `helm` | `helm list -A` |

### Language / ecosystem

| Manager | Command | Ecosystem | List command |
|---|---|---|---|
| Cargo | `cargo` | Rust | `cargo install --list` |
| npm | `npm` | Node.js | `npm -g ls --depth=0` |
| Yarn | `yarn` | Node.js | `yarn global list --depth=0` |
| pnpm | `pnpm` | Node.js | `pnpm -g ls --depth=0` |
| Bun | `bun` | Node.js / Bun | `bun pm -g ls` |
| nvm | `nvm` | Node.js | `nvm list` |
| fnm | `fnm` | Node.js | `fnm list` |
| pip | `pip` | Python | `pip list` |
| pip3 | `pip3` | Python | `pip3 list` |
| pipenv | `pipenv` | Python | — |
| uv | `uv` | Python | `uv tool list` |
| RubyGems | `gem` | Ruby | `gem list` |
| Bundler | `bundle` | Ruby | `bundle list` |
| Composer | `composer` | PHP | `composer global show` |
| dotnet CLI | `dotnet` | .NET | `dotnet tool list -g` |
| Conda | `conda` | Python / data science | `conda list` |
| Go toolchain | `go` | Go | — (project-scoped) |
| Maven | `mvn` | Java | — (project-scoped) |
| Gradle | `gradle` | Java / Kotlin | — (project-scoped) |

> Go, Maven, and Gradle do not have meaningful global package lists — their dependencies are per-project — so `--list` produces no output for them.

## How it works

1. For each known manager, the tool probes `PATH` using `where` (Windows) or `which` (Unix).
2. If found, it runs `<command> <version_flag>` to retrieve the version string, routing through `cmd /C` on Windows so `.cmd` / `.bat` shims (npm, yarn, pnpm, …) resolve correctly.
3. The packages directory is resolved at runtime. Resolution is source-aware: env vars are checked first, then config files are parsed (e.g. `bin` reads `~/.config/bin/config.json` via [simd-json](https://github.com/simd-litmus/simd-json)), then OS-appropriate defaults are used. The source is reported alongside the path.

## Dependencies

| Crate | Purpose |
|---|---|
| [clap 4](https://github.com/clap-rs/clap) | CLI argument parsing |
| [serde](https://serde.rs) | Derive macros for (de)serialisation |
| [serde_json](https://github.com/serde-rs/json) | JSON serialisation for `--json` output |
| [simd-json](https://github.com/simd-litmus/simd-json) | Fast JSON deserialisation for reading manager config files |
