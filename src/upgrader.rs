use colored::*;
use std::process::Command;

pub fn upgrade() {
    let bin_name = std::env::current_exe()
        .ok()
        .and_then(|p| {
            p.file_stem()                          // strip .exe on Windows
                .and_then(|s| s.to_str())
                .map(|s| s.to_string())
        })
        .unwrap_or_else(|| "seekrs".to_string());

    println!();
    println!(
        "  {} {}",
        "🔄".cyan(),
        "Checking for latest version on crates.io…".white().bold()
    );
    println!(
        "  {} Running: {}",
        "⚙️ ".dimmed(),
        format!("cargo install {} --force", bin_name).yellow()
    );
    println!();

    if !cargo_available() {
        println!(
            "  {} {}",
            "❌".red(),
            "cargo not found in PATH.".red().bold()
        );
        println!(
            "  {} Install Rust from {} to enable upgrades.",
            "💡".yellow(),
            "https://rustup.rs".cyan().underline()
        );
        println!(
            "  {} Or download a pre-built binary from the releases page.",
            "💡".yellow()
        );
        return;
    }

    let status = Command::new("cargo")
        .arg("install")
        .arg(&bin_name)
        .arg("--force")
        .status();

    match status {
        Ok(s) if s.success() => {
            println!();
            println!(
                "  {} {} is now up to date!",
                "✅".green(),
                bin_name.white().bold()
            );
            println!(
                "  {} Run {} to confirm.",
                "💡".yellow(),
                format!("{} --version", bin_name).cyan()
            );
        }
        Ok(s) => {
            println!();
            println!(
                "  {} Upgrade failed with exit code: {}",
                "❌".red(),
                s.code().unwrap_or(-1).to_string().yellow()
            );
            println!(
                "  {} You may already be on the latest version, or check your internet connection.",
                "💡".yellow()
            );
        }
        Err(e) => {
            println!(
                "  {} Could not run cargo: {}",
                "❌".red(),
                e.to_string().dimmed()
            );
        }
    }

    println!();
}

fn cargo_available() -> bool {
    Command::new("cargo")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
