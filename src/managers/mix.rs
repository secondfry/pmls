use crate::manager::{Category, PackageManager};

pub fn manager() -> PackageManager {
    PackageManager {
        name: "Mix / Hex (Elixir)",
        command: "mix",
        category: Category::Language,
        version_flag: "--version",
        version_extractor: Some(mix_version),
        config_paths: &[
            "mix.exs",
            "mix.lock",
        ],
        env_vars: &[
            "MIX_HOME",
            "MIX_ARCHIVES",
            "MIX_ENV",
            "HEX_HOME",
        ],
        packages_dir: Some(|env| {
            if let Some(p) = env.get("HEX_HOME") {
                return Some((p.clone(), "$HEX_HOME"));
            }
            if let Some(p) = env.get("MIX_HOME") {
                return Some((p.clone(), "$MIX_HOME"));
            }
            home_dir().map(|h| {
                (std::path::Path::new(&h).join(".hex").to_string_lossy().into_owned(), "default")
            })
        }),
        list_cmd: Some(&["mix", "hex.info"]),
        list_fn: None,
    }
}

/// `mix --version` → "Erlang/OTP 27 [erts-15.1.2]...\nMix 1.17.3 (compiled with Erlang/OTP 27)"
/// Extract the Mix version "1.17.3".
fn mix_version(output: &str) -> Option<String> {
    output.lines()
        .find(|l| l.trim_start().starts_with("Mix "))
        .and_then(|l| l.trim().strip_prefix("Mix "))
        .and_then(|s| s.split_whitespace().next())
        .map(|s| s.to_string())
}

fn home_dir() -> Option<String> {
    #[cfg(windows)]
    return std::env::var("USERPROFILE").ok();
    #[cfg(not(windows))]
    return std::env::var("HOME").ok();
}
