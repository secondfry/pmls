pub mod apt;
pub mod apt_get;
pub mod apk;
pub mod asdf;
pub mod bin;
pub mod brew;
pub mod bun;
pub mod bundler;
pub mod cabal;
pub mod cargo_pm;
pub mod chocolatey;
pub mod composer;
pub mod conan;
pub mod conda;
pub mod corepack;
pub mod coursier;
pub mod cpanm;
pub mod deno;
pub mod dnf;
pub mod dotnet;
pub mod emerge;
pub mod eopkg;
pub mod flatpak;
pub mod flutter;
pub mod fnm;
pub mod gem;
pub mod golang;
pub mod goenv;
pub mod gradle;
pub mod guix;
pub mod hatch;
pub mod helm;
pub mod jenv;
pub mod leiningen;
pub mod luarocks;
pub mod macports;
pub mod mamba;
pub mod maven;
pub mod mill;
pub mod mint;
pub mod mise;
pub mod mix;
pub mod nala;
pub mod nimble;
pub mod nix;
pub mod nodenv;
pub mod npm;
pub mod nuget;
pub mod nvm;
pub mod opam;
pub mod pacman;
pub mod pdm;
pub mod pip;
pub mod pip3;
pub mod pixi;
pub mod pipenv;
pub mod pkg;
pub mod pkgin;
pub mod pnpm;
pub mod poetry;
pub mod pyenv;
pub mod rbenv;
pub mod rustup;
pub mod rvm;
pub mod sbt;
pub mod scoop;
pub mod sdkman;
pub mod snap;
pub mod stack;
pub mod uv;
pub mod vcpkg;
pub mod volta;
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
        guix::manager(),
        macports::manager(),
        nala::manager(),
        nuget::manager(),
        pacman::manager(),
        pkg::manager(),
        pkgin::manager(),
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
    vec![
        bun::manager(),
        corepack::manager(),
        deno::manager(),
        fnm::manager(),
        nodenv::manager(),
        npm::manager(),
        nvm::manager(),
        pnpm::manager(),
        volta::manager(),
        yarn::manager(),
    ]
}

pub fn python() -> Vec<PackageManager> {
    vec![
        conda::manager(),
        hatch::manager(),
        mamba::manager(),
        pdm::manager(),
        pip::manager(),
        pip3::manager(),
        pipenv::manager(),
        pixi::manager(),
        poetry::manager(),
        pyenv::manager(),
        uv::manager(),
    ]
}

pub fn ruby() -> Vec<PackageManager> {
    vec![bundler::manager(), gem::manager(), rbenv::manager(), rvm::manager()]
}

pub fn php() -> Vec<PackageManager> {
    vec![composer::manager()]
}

pub fn dotnet_tools() -> Vec<PackageManager> {
    vec![dotnet::manager()]
}

pub fn rust_tools() -> Vec<PackageManager> {
    vec![cargo_pm::manager(), rustup::manager()]
}

pub fn java_tools() -> Vec<PackageManager> {
    vec![
        coursier::manager(),
        gradle::manager(),
        jenv::manager(),
        leiningen::manager(),
        maven::manager(),
        mill::manager(),
        sbt::manager(),
        sdkman::manager(),
    ]
}

pub fn go_tools() -> Vec<PackageManager> {
    vec![goenv::manager(), golang::manager()]
}

pub fn c_tools() -> Vec<PackageManager> {
    vec![conan::manager(), vcpkg::manager()]
}

pub fn elixir_tools() -> Vec<PackageManager> {
    vec![mix::manager()]
}

pub fn haskell_tools() -> Vec<PackageManager> {
    vec![cabal::manager(), stack::manager()]
}

pub fn ocaml_tools() -> Vec<PackageManager> {
    vec![opam::manager()]
}

pub fn nim_tools() -> Vec<PackageManager> {
    vec![nimble::manager()]
}

pub fn lua_tools() -> Vec<PackageManager> {
    vec![luarocks::manager()]
}

pub fn perl_tools() -> Vec<PackageManager> {
    vec![cpanm::manager()]
}

pub fn dart_tools() -> Vec<PackageManager> {
    vec![flutter::manager()]
}

pub fn swift_tools() -> Vec<PackageManager> {
    vec![mint::manager()]
}

/// Polyglot version / toolchain managers.
// Not used by the CLI (which builds groups inline in main); exposed for
// library consumers who want to enumerate managers by category.
#[allow(dead_code)]
pub fn toolchain() -> Vec<PackageManager> {
    vec![
        asdf::manager(),
        goenv::manager(),
        jenv::manager(),
        mise::manager(),
        nodenv::manager(),
        pyenv::manager(),
        rbenv::manager(),
        rustup::manager(),
        rvm::manager(),
        sdkman::manager(),
        volta::manager(),
    ]
}

/// All language managers concatenated — used by all() and --json output.
// Not used by the CLI directly; exposed for library consumers and used by all().
#[allow(dead_code)]
pub fn language() -> Vec<PackageManager> {
    [
        javascript(),
        python(),
        ruby(),
        php(),
        dotnet_tools(),
        rust_tools(),
        java_tools(),
        go_tools(),
        c_tools(),
        elixir_tools(),
        haskell_tools(),
        ocaml_tools(),
        nim_tools(),
        lua_tools(),
        perl_tools(),
        dart_tools(),
        swift_tools(),
    ]
    .concat()
}

/// Universal / cross-platform package managers (sorted by command).
pub fn universal() -> Vec<PackageManager> {
    vec![
        asdf::manager(),
        helm::manager(),
        mise::manager(),
        nix::manager(),
    ]
}

/// Full catalog — all categories concatenated.
// Not used by the CLI (which builds category groups explicitly); exposed for
// library consumers who want the complete flat manager list in one call.
#[allow(dead_code)]
pub fn all() -> Vec<PackageManager> {
    [system(), language(), universal()].concat()
}
