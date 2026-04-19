use crate::cli::Args;
use crate::display;
use colored::*;
use std::fs;
use std::path::Path;

pub fn print_tree(args: &Args) {
    display::print_section("Directory Tree", "🌳");

    let root = Path::new(&args.dir);

    println!("  {}", root.display().to_string().cyan().bold());
    print_dir(root, "", 0, args);
    println!();
}

fn print_dir(path: &Path, prefix: &str, depth: usize, args: &Args) {
    // Stop at depth 10 to avoid infinite loops with symlinks
    if depth > 10 { return; }

    // Read the directory entries
    let mut entries: Vec<_> = match fs::read_dir(path) {
        Ok(rd) => rd.filter_map(|e| e.ok()).collect(),
        Err(_) => return,
    };

    // Sort directories first, then files, both alphabetically
    entries.sort_by(|a, b| {
        let a_dir = a.path().is_dir();
        let b_dir = b.path().is_dir();
        match (a_dir, b_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.file_name().cmp(&b.file_name()),
        }
    });

    // Filter hidden files (starting with .)
    let entries: Vec<_> = entries
        .into_iter()
        .filter(|e| !e.file_name().to_str().unwrap_or("").starts_with('.'))
        .collect();

    let count = entries.len();

    for (i, entry) in entries.iter().enumerate() {
        let is_last = i == count - 1;
        let entry_path = entry.path();
        let name = entry.file_name();
        let name_str = name.to_str().unwrap_or("?");

        let connector = if is_last { "└── " } else { "├── " };
        let next_prefix = format!("{}{}",
            prefix,
            if is_last { "    " } else { "│   " }
        );

        if entry_path.is_dir() {
            println!(
                "  {}{}{}",
                prefix.dimmed(),
                connector.dimmed(),
                format!("📁 {}/", name_str).cyan().bold()
            );

            if args.ext.is_empty() {
                print_dir(&entry_path, &next_prefix, depth + 1, args);
            } else {
                print_dir(&entry_path, &next_prefix, depth + 1, args);
            }
        } else {
            // Skip files not matching extension filter (if set)
            if !args.ext.is_empty() {
                let ext = entry_path.extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("");
                if !args.ext.iter().any(|f| f.eq_ignore_ascii_case(ext)) {
                    continue;
                }
            }

            // Get file size
            let size_str = entry.metadata()
                .map(|m| format_size(m.len()))
                .unwrap_or_default();

            // Pick icon
            let icon = get_icon(&entry_path);

            println!(
                "  {}{}{}  {}",
                prefix.dimmed(),
                connector.dimmed(),
                format!("{} {}", icon, name_str).white(),
                format!("[{}]", size_str).green().dimmed()
            );
        }
    }
}

fn get_icon(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("rs")  => "🦀",
        Some("py")  => "🐍",
        Some("js") | Some("ts") => "⚡",
        Some("md")  => "📝",
        Some("toml") | Some("yaml") | Some("yml") | Some("json") => "⚙️ ",
        Some("html") => "🌐",
        Some("css")  => "🎨",
        Some("lock") => "🔒",
        Some("sh")   => "🖥️ ",
        _            => "📄",
    }
}

fn format_size(bytes: u64) -> String {
    if bytes < 1024 { format!("{}B", bytes) }
    else if bytes < 1024*1024 { format!("{:.1}KB", bytes as f64/1024.0) }
    else { format!("{:.1}MB", bytes as f64/(1024.0*1024.0)) }
}
