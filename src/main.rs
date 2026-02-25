mod detect;
mod manager;
mod managers;

use clap::Parser;
use colored::Colorize;
use manager::JsonEntry;
use managers::{system, universal, javascript, python, ruby, php, dotnet_tools, rust_tools, java_tools, go_tools};

#[derive(Parser)]
#[command(name = "detector", about = "Detect installed package managers")]
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
    let groups = [
        ("System",      detect::detect(system())),
        ("JavaScript",  detect::detect(javascript())),
        ("Python",      detect::detect(python())),
        ("Ruby",        detect::detect(ruby())),
        ("PHP",         detect::detect(php())),
        (".NET",        detect::detect(dotnet_tools())),
        ("Rust",        detect::detect(rust_tools())),
        ("Java",        detect::detect(java_tools())),
        ("Go",          detect::detect(go_tools())),
        ("Universal",   detect::detect(universal())),
    ];

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
            match &d.packages_dir {
                Some(dir) => println!("{} {} {} {} {} {} {}", cmd, sep, name, sep, version, sep, dir.dimmed()),
                None      => println!("{} {} {} {} {}", cmd, sep, name, sep, version),
            }

            if list {
                match d.manager.list_cmd {
                    None => {
                        if verbose {
                            eprintln!("  {}", format!("(no list command for {})", d.manager.name).dimmed());
                        }
                    }
                    Some(cmd) => match detect::run_list(cmd) {
                        Ok(lines) => {
                            for line in &lines {
                                println!("  {}", line.dimmed());
                            }
                        }
                        Err(e) => {
                            if verbose {
                                eprintln!("  {}", format!("error listing {}: {}", d.manager.name, e).red());
                            }
                        }
                    },
                }
            }
        }
    }
    println!();
}

// ── JSON output ───────────────────────────────────────────────────────────────

fn print_json(list: bool, verbose: bool) {
    let detected = detect::detect(managers::all());
    let entries: Vec<JsonEntry> = detected
        .iter()
        .map(|d| {
            let (packages, list_error) = if list {
                match d.manager.list_cmd {
                    None => (None, None),
                    Some(cmd) => match detect::run_list(cmd) {
                        Ok(lines) => (Some(lines), None),
                        Err(e) => {
                            let err = if verbose { Some(e) } else { None };
                            (None, err)
                        }
                    },
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
                packages,
                list_error,
            }
        })
        .collect();

    println!("{}", serde_json::to_string_pretty(&entries).unwrap());
}
