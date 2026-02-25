# detect-package-managers

A fast, cross-platform CLI tool written in Rust that detects which package managers are installed on the current machine, reports their version and global packages directory, and can enumerate what they have installed.

## Usage

```
detector [OPTIONS]
```

| Flag | Short | Description |
|---|---|---|
| `--list` | `-l` | Run each manager's list command and print installed packages |
| `--json` | `-j` | Output results as a JSON array |
| `--verbose` | `-v` | Show errors when package listing fails |

Flags can be combined freely, e.g. `-lj` produces JSON that includes the installed package list.

### Default output

```
# System
Scoop # v0.5.3 # C:\Users\you\scoop\apps
Winget # v1.12.460 # C:\Users\you\AppData\Local\Microsoft\WinGet\Packages
bin # 0.23.1 # D:\Development\bin

# Language
Cargo (Rust) # 1.85.0 # D:\rust\cargo
npm (Node.js) # 10.9.0 # C:\Users\you\AppData\Roaming\npm\node_modules
Bun # 1.3.0 # C:\Users\you\.bun
```

Each line follows the format `name # version # packages_dir`. The packages directory is omitted when the manager does not have a fixed global location.

### `--list`

```
# System
Scoop # v0.5.3 # C:\Users\secon\scoop\apps
  There aren't any apps installed.
bin # 0.23.1 # D:\Development\bin
  Path                             Version  URL                                Status
  D:\Development\bin\bin.exe       v0.23.1  github.com/marcosnils/bin          OK
  D:\Development\bin\doggo.exe     v1.1.2   https://github.com/mr-karan/doggo  OK
  D:\Development\bin\instawow.exe  v6.4.1   github.com/layday/instawow         OK
  D:\Development\bin\mkcert.exe    v1.4.4   github.com/FiloSottile/mkcert      OK
  D:\Development\bin\pandoc.exe    3.8.2.1  github.com/jgm/pandoc              OK
  D:\Development\bin\whois.exe     v1.15.6  github.com/likexian/whois          OK
```

### `--json`

```json
[
  {
    "name": "Scoop",
    "category": "System",
    "version": "v0.5.3",
    "packages_dir": "C:\\Users\\you\\scoop\\apps"
  },
  {
    "name": "npm (Node.js)",
    "category": "Language",
    "version": "10.9.0",
    "packages_dir": "C:\\Users\\you\\AppData\\Roaming\\npm\\node_modules"
  }
]
```

With `--list --json` each entry gains a `"packages"` array (and `"list_error"` when `--verbose` is set and listing failed).

## Build

Requires a stable Rust toolchain (edition 2024).

```sh
cargo build --release
# binary at target/release/detector  (detector.exe on Windows)
```

## Supported package managers

### System — Windows

| Manager | List command |
|---|---|
| Chocolatey | `choco list --local-only` |
| Scoop | `scoop list` |
| winget | `winget list --disable-interactivity` |
| NuGet CLI | — |

### System — Linux

| Manager | List command |
|---|---|
| apt | `apt list --installed` |
| apt-get | `apt list --installed` |
| pacman | `pacman -Q` |
| dnf | `dnf list installed` |
| yum | `yum list installed` |
| zypper | `zypper packages --installed-only` |
| apk | `apk list --installed` |
| snap | `snap list` |
| Flatpak | `flatpak list` |
| Portage (emerge) | — |
| eopkg | `eopkg list-installed` |
| xbps | `xbps-query -l` |

### System — macOS

| Manager | List command |
|---|---|
| Homebrew | `brew list` |
| MacPorts | `port installed` |

### System — cross-platform

| Manager | List command |
|---|---|
| bin | `bin ls` |

### Universal

| Manager | List command |
|---|---|
| Nix | `nix profile list` |
| Helm | `helm list -A` |

### Language / ecosystem

| Manager | Ecosystem | List command |
|---|---|---|
| Cargo | Rust | `cargo install --list` |
| npm | Node.js | `npm -g ls --depth=0` |
| Yarn | Node.js | `yarn global list --depth=0` |
| pnpm | Node.js | `pnpm -g ls --depth=0` |
| Bun | Node.js / Bun | `bun pm -g ls` |
| pip | Python | `pip list` |
| pip3 | Python | `pip3 list` |
| uv | Python | `uv tool list` |
| RubyGems | Ruby | `gem list` |
| Bundler | Ruby | `bundle list` |
| Composer | PHP | `composer global show` |
| dotnet CLI | .NET | `dotnet tool list -g` |
| Conda | Python / data science | `conda list` |
| Go toolchain | Go | — (project-scoped) |
| Maven | Java | — (project-scoped) |
| Gradle | Java / Kotlin | — (project-scoped) |

> Go, Maven, and Gradle do not have meaningful global package lists — their dependencies are per-project — so `--list` produces no output for them.

## How it works

1. For each known manager, the tool probes the `PATH` using `where` (Windows) or `which` (Unix).
2. If found, it runs `<command> <version_flag>` to retrieve the version string, routing through `cmd /C` on Windows so `.cmd` / `.bat` shims (npm, yarn, pnpm, …) resolve correctly.
3. The packages directory is resolved at runtime; for managers that store it in a config file (e.g. `bin` reads `~/.config/bin/config.json`) that file is parsed with [simd-json](https://github.com/simd-litmus/simd-json).

## Dependencies

| Crate | Purpose |
|---|---|
| [clap 4](https://github.com/clap-rs/clap) | CLI argument parsing |
| [serde](https://serde.rs) | derive macros for (de)serialisation |
| [serde_json](https://github.com/serde-rs/json) | JSON serialisation for `--json` output |
| [simd-json](https://github.com/simd-litmus/simd-json) | Fast JSON deserialisation for reading manager config files |
