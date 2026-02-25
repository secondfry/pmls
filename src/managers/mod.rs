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
pub fn language() -> Vec<PackageManager> {
    vec![
        bun::manager(),
        bundler::manager(),
        cargo_pm::manager(),
        composer::manager(),
        conda::manager(),
        dotnet::manager(),
        gem::manager(),
        golang::manager(),
        gradle::manager(),
        maven::manager(),
        npm::manager(),
        pip::manager(),
        pip3::manager(),
        pnpm::manager(),
        uv::manager(),
        yarn::manager(),
    ]
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
