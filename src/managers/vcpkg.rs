use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "vcpkg",
        command: "vcpkg",
        category: Category::Language,
        version_flag: "version",
        version_extractor: Some(vcpkg_version),
        config_paths: &[
            "vcpkg.json",
            "vcpkg-configuration.json",
        ],
        env_vars: &[
            "VCPKG_ROOT",
            "VCPKG_DEFAULT_TRIPLET",
            "VCPKG_FEATURE_FLAGS",
            "VCPKG_DOWNLOADS",
        ],
        packages_dir: Some(|env| {
            if let Some(p) = env.get("VCPKG_ROOT") {
                return Some((
                    std::path::Path::new(p).join("installed").to_string_lossy().into_owned(),
                    "$VCPKG_ROOT/installed",
                ));
            }
            None
        }),
        list_cmd: Some(&["vcpkg", "list"]),
        list_fn: None,
    }
}

/// `vcpkg version` → "vcpkg package management program version 2024-10-07-..." — extract the date portion.
fn vcpkg_version(output: &str) -> Option<String> {
    let line = output.lines().find(|l| l.contains("version"))?;
    // last token is the full version string like "2024-10-07-abc123"
    line.split_whitespace().last().map(|s| s.to_string())
}
