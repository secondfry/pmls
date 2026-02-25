mod detect;
mod manager;
mod managers;

use clap::Parser;
use colored::Colorize;
use manager::JsonEntry;
use managers::{
    system, universal,
    javascript, python, ruby, php, dotnet_tools, rust_tools, java_tools, go_tools,
    c_tools, elixir_tools, haskell_tools, ocaml_tools, nim_tools, lua_tools,
    perl_tools, dart_tools, swift_tools,
};

/// The canonical ordered list of display groups, used by both human and JSON output.
fn labeled_groups() -> Vec<(&'static str, Vec<manager::PackageManager>)> {
    vec![
        ("System",      system()),
        ("JavaScript",  javascript()),
        ("Python",      python()),
        ("Ruby",        ruby()),
        ("PHP",         php()),
        (".NET",        dotnet_tools()),
        ("Rust",        rust_tools()),
        ("Java",        java_tools()),
        ("Go",          go_tools()),
        ("C/C++",       c_tools()),
        ("Elixir",      elixir_tools()),
        ("Haskell",     haskell_tools()),
        ("OCaml",       ocaml_tools()),
        ("Nim",         nim_tools()),
        ("Lua",         lua_tools()),
        ("Perl",        perl_tools()),
        ("Dart",        dart_tools()),
        ("Swift",       swift_tools()),
        ("Universal",   universal()),
    ]
}

#[derive(Parser)]
#[command(name = "pmls", about = "List installed package managers")]
struct Cli {
    /// List installed packages for each detected manager
    #[arg(short, long)]
    list: bool,

    /// Output results as JSON
    #[arg(short, long)]
    json: bool,

    /// Show errors when package listing fails
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.json {
        print_json(cli.list, cli.verbose);
    } else {
        print_human(cli.list, cli.verbose);
    }
}

// ── Human-readable output ─────────────────────────────────────────────────────

fn print_human(list: bool, verbose: bool) {
    // One parallel pass over all 77+ managers at once.
    let groups = detect::detect_grouped(labeled_groups());

    let mut first = true;
    for (label, group) in &groups {
        if group.is_empty() {
            continue;
        }
        if !first {
            println!();
        }
        first = false;
        println!("{}", format!("# {label}").cyan().bold());
        for d in group {
            let sep = "#".dimmed();
            let cmd = d.manager.command.bold();
            let name = d.manager.name.dimmed();
            let version = d.version.as_deref().unwrap_or("unknown").yellow();
            match (&d.packages_dir, d.packages_dir_source) {
                (Some(dir), Some(src)) => println!("{} {} {} {} {} {} {} ({})", cmd, sep, name, sep, version, sep, dir.dimmed(), src.dimmed()),
                (Some(dir), None)      => println!("{} {} {} {} {} {} {}", cmd, sep, name, sep, version, sep, dir.dimmed()),
                _                      => println!("{} {} {} {} {}", cmd, sep, name, sep, version),
            }

            if list {
                let env_map: manager::EnvMap = d.manager.env_vars.iter()
                    .filter_map(|k| std::env::var(k).ok().map(|v| (*k, v)))
                    .collect();
                let result = if let Some(f) = d.manager.list_fn {
                    Some(f(&env_map))
                } else {
                    d.manager.list_cmd.map(detect::run_list)
                };
                match result {
                    None => {
                        if verbose {
                            eprintln!("  {}", format!("(no list command for {})", d.manager.name).dimmed());
                        }
                    }
                    Some(Ok(lines)) => {
                        for line in &lines {
                            println!("  {}", line.dimmed());
                        }
                    }
                    Some(Err(e)) => {
                        if verbose {
                            eprintln!("  {}", format!("error listing {}: {}", d.manager.name, e).red());
                        }
                    }
                }
            }
        }
    }
    println!();
}

// ── JSON output ───────────────────────────────────────────────────────────────

fn print_json(list: bool, verbose: bool) {
    // Flatten the grouped results into a single ordered list for JSON.
    let detected: Vec<_> = detect::detect_grouped(labeled_groups())
        .into_iter()
        .flat_map(|(_, group)| group)
        .collect();
    let entries: Vec<JsonEntry> = detected
        .iter()
        .map(|d| {
            let (packages, list_error) = if list {
                let env_map: manager::EnvMap = d.manager.env_vars.iter()
                    .filter_map(|k| std::env::var(k).ok().map(|v| (*k, v)))
                    .collect();
                let result = if let Some(f) = d.manager.list_fn {
                    Some(f(&env_map))
                } else {
                    d.manager.list_cmd.map(detect::run_list)
                };
                match result {
                    None => (None, None),
                    Some(Ok(lines)) => (Some(lines), None),
                    Some(Err(e)) => {
                        let err = if verbose { Some(e) } else { None };
                        (None, err)
                    }
                }
            } else {
                (None, None)
            };

            JsonEntry {
                command: d.manager.command.to_string(),
                name: d.manager.name.to_string(),
                category: d.manager.category.to_string(),
                version: d.version.clone(),
                packages_dir: d.packages_dir.clone(),
                packages_dir_source: d.packages_dir_source.map(|s| s.to_string()),
                packages,
                list_error,
            }
        })
        .collect();

    println!("{}", serde_json::to_string_pretty(&entries).unwrap());
}
