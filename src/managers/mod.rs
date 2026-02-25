pub mod apt;
pub mod apt_get;
pub mod apk;
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
        // ── Windows ──────────────────────────────────────────────────────────
        chocolatey::manager(),
        scoop::manager(),
        winget::manager(),
        nuget::manager(),
        // ── Linux ─────────────────────────────────────────────────────────────
        apt::manager(),
        apt_get::manager(),
        pacman::manager(),
        dnf::manager(),
        yum::manager(),
        zypper::manager(),
        apk::manager(),
        snap::manager(),
        flatpak::manager(),
        emerge::manager(),
        eopkg::manager(),
        xbps::manager(),
        // ── macOS ─────────────────────────────────────────────────────────────
        brew::manager(),
        macports::manager(),
        // ── Universal ─────────────────────────────────────────────────────────
        nix::manager(),
        helm::manager(),
        // ── Language / ecosystem ──────────────────────────────────────────────
        cargo_pm::manager(),
        npm::manager(),
        yarn::manager(),
        pnpm::manager(),
        bun::manager(),
        pip::manager(),
        pip3::manager(),
        uv::manager(),
        gem::manager(),
        bundler::manager(),
        composer::manager(),
        golang::manager(),
        maven::manager(),
        gradle::manager(),
        dotnet::manager(),
        conda::manager(),
    ]
}
