pub mod apt;
pub mod apt_get;
pub mod apk;
pub mod bin;
pub mod brew;
pub mod bun;
pub mod bundler;
pub mod cargo_pm;
pub mod chocolatey;
pub mod composer;
pub mod conda;
pub mod dnf;
pub mod dotnet;
pub mod emerge;
pub mod eopkg;
pub mod flatpak;
pub mod gem;
pub mod golang;
pub mod gradle;
pub mod helm;
pub mod macports;
pub mod maven;
pub mod nix;
pub mod npm;
pub mod nuget;
pub mod pacman;
pub mod pip;
pub mod pip3;
pub mod pnpm;
pub mod scoop;
pub mod snap;
pub mod uv;
pub mod winget;
pub mod xbps;
pub mod yarn;
pub mod yum;
pub mod zypper;

use crate::manager::PackageManager;

/// System-level package managers (sorted by command).
pub fn system() -> Vec<PackageManager> {
    vec![
        apk::manager(),
        apt::manager(),
        apt_get::manager(),
        bin::manager(),
        brew::manager(),
        chocolatey::manager(),
        dnf::manager(),
        emerge::manager(),
        eopkg::manager(),
        flatpak::manager(),
        macports::manager(),
        nuget::manager(),
        pacman::manager(),
        scoop::manager(),
        snap::manager(),
        winget::manager(),
        xbps::manager(),
        yum::manager(),
        zypper::manager(),
    ]
}

/// Language / ecosystem package managers (sorted by command).
pub fn javascript() -> Vec<PackageManager> {
    vec![bun::manager(), npm::manager(), pnpm::manager(), yarn::manager()]
}

pub fn python() -> Vec<PackageManager> {
    vec![conda::manager(), pip::manager(), pip3::manager(), uv::manager()]
}

pub fn ruby() -> Vec<PackageManager> {
    vec![bundler::manager(), gem::manager()]
}

pub fn php() -> Vec<PackageManager> {
    vec![composer::manager()]
}

pub fn dotnet_tools() -> Vec<PackageManager> {
    vec![dotnet::manager()]
}

pub fn rust_tools() -> Vec<PackageManager> {
    vec![cargo_pm::manager()]
}

pub fn java_tools() -> Vec<PackageManager> {
    vec![gradle::manager(), maven::manager()]
}

pub fn go_tools() -> Vec<PackageManager> {
    vec![golang::manager()]
}

/// All language managers concatenated — used by all() and --json output.
pub fn language() -> Vec<PackageManager> {
    [javascript(), python(), ruby(), php(), dotnet_tools(), rust_tools(), java_tools(), go_tools()].concat()
}

/// Universal / cross-platform package managers (sorted by command).
pub fn universal() -> Vec<PackageManager> {
    vec![
        helm::manager(),
        nix::manager(),
    ]
}

/// Full catalog — all categories concatenated.
pub fn all() -> Vec<PackageManager> {
    [system(), language(), universal()].concat()
}
