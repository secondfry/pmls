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

/// Returns the full catalog of known package managers.
pub fn all() -> Vec<PackageManager> {
    vec![
        // ── System (apk … zypper, sorted by command) ─────────────────────────
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
        nuget::manager(),
        pacman::manager(),
        macports::manager(),
        scoop::manager(),
        snap::manager(),
        winget::manager(),
        xbps::manager(),
        yum::manager(),
        zypper::manager(),
        // ── Language / ecosystem (bun … yarn, sorted by command) ─────────────
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
        // ── Universal (helm … nix, sorted by command) ─────────────────────────
        helm::manager(),
        nix::manager(),
    ]
}
