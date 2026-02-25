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
| Windows x86-64 | `pmls_<version>_windows_amd64.exe` |
| Linux x86-64 | `pmls_<version>_linux_amd64` |
| macOS x86-64 | `pmls_<version>_darwin_amd64` |
| macOS arm64 | `pmls_<version>_darwin_arm64` |

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
| Nala | `nala` | `nala list --installed` |
| GNU Guix | `guix` | `guix package --list-installed` |

### System — FreeBSD / Termux

| Manager | Command | List command |
|---|---|---|
| pkg | `pkg` | `pkg info` |
| pkgin | `pkgin` | `pkgin list` |

### System — macOS

| Manager | Command | List command |
|---|---|---|
| Homebrew | `brew` | `brew list` |
| MacPorts | `port` | `port installed` |

### System — cross-platform

| Manager | Command | List command |
|---|---|---|
| bin | `bin` | `bin ls` |

### Universal / polyglot

| Manager | Command | List command |
|---|---|---|
| Nix | `nix` | `nix profile list` |
| Helm | `helm` | `helm list -A` |
| asdf | `asdf` | `asdf list` |
| mise | `mise` | `mise ls` |

### Toolchain / version managers

These managers govern which runtime version is active rather than installing packages; their `--list` output enumerates installed runtime versions.

| Manager | Command | Ecosystem | List command |
|---|---|---|---|
| rustup | `rustup` | Rust | `rustup toolchain list` |
| pyenv | `pyenv` | Python | `pyenv versions` |
| rbenv | `rbenv` | Ruby | `rbenv versions` |
| RVM | `rvm` | Ruby | `rvm list` |
| Volta | `volta` | Node.js | `volta list --format plain` |
| nodenv | `nodenv` | Node.js | `nodenv versions` |
| SDKMAN! | `sdk` | JVM | `sdk list` |
| jenv | `jenv` | Java | `jenv versions` |
| goenv | `goenv` | Go | `goenv versions` |

### Language / ecosystem

#### Node.js / JavaScript

| Manager | Command | List command |
|---|---|---|
| npm | `npm` | `npm -g ls --depth=0` |
| Yarn | `yarn` | `yarn global list --depth=0` |
| pnpm | `pnpm` | `pnpm -g ls --depth=0` |
| Bun | `bun` | `bun pm -g ls` |
| Deno | `deno` | `deno info` |
| nvm | `nvm` | `nvm list` |
| fnm | `fnm` | `fnm list` |
| Corepack | `corepack` | — |

#### Python

| Manager | Command | List command |
|---|---|---|
| pip | `pip` | `pip list` |
| pip3 | `pip3` | `pip3 list` |
| pipenv | `pipenv` | — |
| uv | `uv` | `uv tool list` |
| Poetry | `poetry` | `poetry env list` |
| PDM | `pdm` | `pdm list` |
| Hatch | `hatch` | `hatch env show` |
| Conda | `conda` | `conda list` |
| Mamba | `mamba` | `mamba list` |
| pixi | `pixi` | `pixi list` |

#### Ruby

| Manager | Command | List command |
|---|---|---|
| RubyGems | `gem` | `gem list` |
| Bundler | `bundle` | `bundle list` |

#### PHP

| Manager | Command | List command |
|---|---|---|
| Composer | `composer` | `composer global show` |

#### .NET

| Manager | Command | List command |
|---|---|---|
| dotnet CLI | `dotnet` | `dotnet tool list -g` |

#### Rust

| Manager | Command | List command |
|---|---|---|
| Cargo | `cargo` | `cargo install --list` |
| rustup | `rustup` | `rustup toolchain list` |

#### Go

| Manager | Command | List command |
|---|---|---|
| Go toolchain | `go` | `go version -m` on each binary in GOBIN |
| goenv | `goenv` | `goenv versions` |

#### Java / JVM

| Manager | Command | List command |
|---|---|---|
| Maven | `mvn` | — (project-scoped) |
| Gradle | `gradle` | — (project-scoped) |
| sbt | `sbt` | — (project-scoped) |
| Leiningen | `lein` | — (project-scoped) |
| Coursier | `cs` | `cs list` |
| Mill | `mill` | — (project-scoped) |
| SDKMAN! | `sdk` | `sdk list` |
| jenv | `jenv` | `jenv versions` |

#### C / C++

| Manager | Command | List command |
|---|---|---|
| vcpkg | `vcpkg` | `vcpkg list` |
| Conan | `conan` | `conan list` |

#### Elixir

| Manager | Command | List command |
|---|---|---|
| Mix / Hex | `mix` | `mix hex.info` |

#### Haskell

| Manager | Command | List command |
|---|---|---|
| Stack | `stack` | `stack ls dependencies` |
| Cabal | `cabal` | `cabal list --installed` |

#### OCaml

| Manager | Command | List command |
|---|---|---|
| opam | `opam` | `opam list` |

#### Nim

| Manager | Command | List command |
|---|---|---|
| Nimble | `nimble` | `nimble list --installed` |

#### Lua

| Manager | Command | List command |
|---|---|---|
| LuaRocks | `luarocks` | `luarocks list` |

#### Perl

| Manager | Command | List command |
|---|---|---|
| cpanminus | `cpanm` | — |

#### Dart / Flutter

| Manager | Command | List command |
|---|---|---|
| Flutter | `flutter` | — |

#### Swift

| Manager | Command | List command |
|---|---|---|
| Mint | `mint` | `mint list` |

> Maven, Gradle, sbt, Leiningen, and Mill do not have meaningful global package lists — their dependencies are per-project — so `--list` produces no output for them.

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
