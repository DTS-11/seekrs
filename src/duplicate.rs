use crate::cli::Args;
use crate::display;
use colored::*;
use md5;
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn find_duplicates(args: &Args) {
    display::print_section("Duplicate File Detector", "👯");

    // HashMap: hash_string → list of paths with that hash
    let mut hash_map: HashMap<String, Vec<PathBuf>> = HashMap::new();

    let mut scanned = 0usize;

    println!("  {} Hashing files…", "⏳".yellow());

    for entry in WalkDir::new(&args.dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        // Only hash regular files (skip dirs, symlinks, etc.)
        if !path.is_file() { continue; }

        // Skip very large files (>50MB) to keep things fast
        if let Ok(meta) = entry.metadata() {
            if meta.len() > 50 * 1024 * 1024 { continue; }
        }

        if let Some(hash) = compute_md5(path) {
            hash_map
                .entry(hash)
                .or_insert_with(Vec::new)
                .push(path.to_path_buf());
            scanned += 1;
        }
    }

    // Filter to only groups with more than one file (actual duplicates)
    let duplicates: Vec<(&String, &Vec<PathBuf>)> = hash_map
        .iter()
        .filter(|(_, paths)| paths.len() > 1)
        .collect();

    if duplicates.is_empty() {
        display::print_info("🎉 No duplicates found! Your files are all unique.");
    } else {
        // Calculate total wasted space
        let mut total_wasted: u64 = 0;

        for (hash, paths) in &duplicates {
            let file_size = fs::metadata(&paths[0]).map(|m| m.len()).unwrap_or(0);
            // Wasted = (copies - 1) × size
            let wasted = file_size * (paths.len() as u64 - 1);
            total_wasted += wasted;

            display::print_duplicate_header(hash, paths.len(), file_size);

            for (i, path) in paths.iter().enumerate() {
                let marker = if i == 0 {
                    "  ✅ Keep:   ".green().bold().to_string()
                } else {
                    "  🗑️  Dupe:   ".red().to_string()
                };
                println!("{}{}", marker, path.display().to_string().white());
            }
        }

        println!("\n  {} {} duplicate group{} found across {} files",
            "📊".cyan(),
            duplicates.len().to_string().yellow().bold(),
            if duplicates.len() == 1 { "" } else { "s" },
            scanned.to_string().cyan()
        );
        println!("  {} Potential space savings: {}",
            "💾".green(),
            format_size(total_wasted).green().bold()
        );
    }

    println!("  {} Scanned {} files total\n", "✅".green(), scanned.to_string().cyan());
}

fn compute_md5(path: &std::path::Path) -> Option<String> {
    let mut file = fs::File::open(path).ok()?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).ok()?;

    // md5::compute returns a Digest, format! with {:x} gives hex string
    let digest = md5::compute(&buffer);
    Some(format!("{:x}", digest))
}

fn format_size(bytes: u64) -> String {
    if bytes < 1024 { format!("{}B", bytes) }
    else if bytes < 1024*1024 { format!("{:.1}KB", bytes as f64 / 1024.0) }
    else if bytes < 1024*1024*1024 { format!("{:.1}MB", bytes as f64 / (1024.0*1024.0)) }
    else { format!("{:.2}GB", bytes as f64 / (1024.0*1024.0*1024.0)) }
}
