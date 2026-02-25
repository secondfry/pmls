mod detect;
mod manager;
mod managers;

use manager::Category;

fn main() {
    let detected = detect::detect(managers::all());

    if detected.is_empty() {
        println!("No package managers detected.");
        return;
    }

    println!("Detected {} package manager(s):\n", detected.len());

    for category in [Category::System, Category::Language, Category::Universal] {
        let group: Vec<_> = detected
            .iter()
            .filter(|d| d.manager.category == category)
            .collect();

        if group.is_empty() {
            continue;
        }

        println!("  [{}]", category);
        for d in &group {
            let version = d.version.as_deref().unwrap_or("version unknown");
            print!("    \u{2713} {} ({})", d.manager.name, version);
            if let Some(dir) = &d.packages_dir {
                print!("  [packages: {}]", dir);
            }
            println!();
        }
        println!();
    }
}
