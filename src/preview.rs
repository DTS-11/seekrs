use colored::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn show_preview(path: &Path, lines: usize) {
    // Try to open the file — if it fails, just skip the preview
    let file = match File::open(path) {
        Ok(f) => f,
        Err(_) => {
            println!("     {} Cannot open file for preview", "⚠️".yellow());
            return;
        }
    };

    // BufReader wraps the file and lets us read line by line efficiently
    let reader = BufReader::new(file);

    println!("     {}", "┌─ Preview ──────────────────────────────────┐".dimmed());

    let mut count = 0;
    for line in reader.lines() {
        if count >= lines { break; }

        // `line` here is a Result<String, Error>
        // `.unwrap_or_default()` gives empty string on error
        let text = line.unwrap_or_default();

        // Truncate very long lines to fit the box
        let display = if text.len() > 52 {
            format!("{}…", &text[..52])
        } else {
            text
        };

        println!("     {}  {}", "│".dimmed(), display.cyan());
        count += 1;
    }

    if count == 0 {
        println!("     {}  {}", "│".dimmed(), "(empty file)".dimmed().italic());
    }

    println!("     {}", "└────────────────────────────────────────────┘".dimmed());
}