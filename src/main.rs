mod detect;
mod manager;
mod managers;

use clap::Parser;
use colored::Colorize;
use manager::{Category, JsonEntry};

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
    let detected = detect::detect(managers::all());

    if detected.is_empty() {
        if cli.json {
            println!("[]");
        } else {
            eprintln!("No package managers detected.");
        }
        return;
    }

    if cli.json {
        print_json(&detected, cli.list, cli.verbose);
    } else {
        print_human(&detected, cli.list, cli.verbose);
    }
}

// ── Human-readable output ─────────────────────────────────────────────────────

fn print_human(
    detected: &[manager::DetectedPackageManager],
    list: bool,
    verbose: bool,
) {
    for category in [Category::System, Category::Language, Category::Universal] {
        let group: Vec<_> = detected
            .iter()
            .filter(|d| d.manager.category == category)
            .collect();

        if group.is_empty() {
            continue;
        }

        println!("{}", format!("# {}", category).cyan().bold());
        for d in &group {
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
        println!();
    }
}

// ── JSON output ───────────────────────────────────────────────────────────────

fn print_json(
    detected: &[manager::DetectedPackageManager],
    list: bool,
    verbose: bool,
) {
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
