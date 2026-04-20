use colored::*;
use std::path::Path;

pub fn print_banner() {
    println!("\n{}", "╔══════════════════════════════════════════════════════╗".cyan().bold());
    println!("{}", "║                                                      ║".cyan().bold());
    println!("{}{}{}",
        "║  ".cyan().bold(),
        " 🔍  S E E K . R S  —  v0.2.0              ".white().bold(),
        "║".cyan().bold()
    );
    println!("{}", "║     blazing fast · colorful · feature-rich           ║".cyan().dimmed());
    println!("{}", "╚══════════════════════════════════════════════════════╝".cyan().bold());
    println!();
}

pub fn print_section(title: &str, emoji: &str) {
    println!("\n  {} {}", emoji, title.bold().underline().yellow());
    println!("  {}", "─".repeat(48).dimmed());
}

pub fn print_result(
    index: usize,
    path: &Path,
    size: u64,
    modified: Option<String>,
    match_info: Option<String>,
) {
    // Determine icon based on file extension
    let icon = get_file_icon(path);

    // Color the file name differently from the parent directory path
    let parent = path.parent()
        .and_then(|p| p.to_str())
        .unwrap_or("");
    let filename = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");

    // Format the size to human-readable (e.g. "1.2KB")
    let size_str = format_size(size);

    // Index counter
    print!("  {}  ", format!("{:>3}.", index).dimmed());

    // Icon + directory path + filename
    print!("{} {}{}", icon, parent.dimmed(), format!("/{}", filename).white().bold());

    // Size badge
    print!("  {}", format!("[{}]", size_str).green());

    // Date if available
    if let Some(date) = modified {
        print!("  {}", format!("📅 {}", date).dimmed());
    }

    // Match context if content search found something
    if let Some(info) = match_info {
        println!();
        println!("     {} {}", "↳".yellow(), info.italic().cyan());
    } else {
        println!();
    }
}

pub fn print_fuzzy_result(index: usize, path: &Path, score: i64, size: u64) {
    let icon = get_file_icon(path);
    let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    let parent = path.parent().and_then(|p| p.to_str()).unwrap_or("");
    let size_str = format_size(size);

    // Show fuzzy score as a visual bar
    let bar_len = (score.max(0).min(100) / 10) as usize;
    let bar = format!("{}{}", "█".repeat(bar_len).green(), "░".repeat(10 - bar_len).dimmed());

    println!(
        "  {:>3}.  {} {}{} {} {} {}",
        index,
        icon,
        parent.dimmed(),
        format!("/{}", filename).white().bold(),
        format!("[{}]", size_str).green(),
        bar,
        format!("{}%", score).yellow().bold()
    );
}

#[allow(dead_code)]
pub fn print_content_match(line_num: usize, line: &str, highlight: &str) {
    // Replace the match with a highlighted version
    let highlighted = line.replace(highlight, &highlight.red().bold().to_string());
    println!(
        "       {} {}  {}",
        format!("L{:<4}", line_num).yellow().bold(),
        "│".dimmed(),
        highlighted
    );
}

pub fn print_duplicate_header(hash: &str, count: usize, size: u64) {
    println!(
        "\n  {} {} {} {} {}",
        "⚠️ ".yellow(),
        "DUPLICATE GROUP".yellow().bold(),
        format!("({} files)", count).white(),
        format!("Hash: {}...", &hash[..8]).dimmed(),
        format!("each {}", format_size(size)).green()
    );
}

pub fn print_summary(total: usize, searched: usize, elapsed_ms: u128) {
    println!("\n  {}", "─".repeat(54).dimmed());
    println!(
        "  {} Found {} in {} files  ⏱  {}ms",
        "✅".green(),
        format!("{} result{}", total, if total == 1 { "" } else { "s" }).white().bold(),
        searched.to_string().cyan(),
        elapsed_ms.to_string().yellow()
    );
    println!();
}

pub fn print_error(msg: &str) {
    eprintln!("  {} {}", "❌".red(), msg.red().bold());
}

pub fn print_info(msg: &str) {
    println!("  {} {}", "ℹ️ ".cyan(), msg.cyan());
}

pub fn print_warning(msg: &str) {
    println!("  {} {}", "⚠️ ".yellow(), msg.yellow());
}

fn get_file_icon(path: &Path) -> &'static str {
    // `match` in Rust is like a powerful switch-case statement.
    // It checks the value and runs the matching arm.
    // The `_` arm is the "default/catch-all" case.
    match path.extension().and_then(|e| e.to_str()) {
        Some("rs")                         => "🦀",
        Some("py")                         => "🐍",
        Some("js") | Some("ts")            => "⚡",
        Some("html") | Some("htm")         => "🌐",
        Some("css") | Some("scss")         => "🎨",
        Some("json") | Some("toml") | Some("yaml") | Some("yml") => "⚙️ ",
        Some("md") | Some("txt")           => "📝",
        Some("pdf")                        => "📄",
        Some("png") | Some("jpg") | Some("jpeg") | Some("gif") | Some("svg") => "🖼️ ",
        Some("mp3") | Some("wav") | Some("flac") => "🎵",
        Some("mp4") | Some("mkv") | Some("avi")  => "🎬",
        Some("zip") | Some("tar") | Some("gz") | Some("rar") => "📦",
        Some("sh") | Some("bash")          => "🖥️ ",
        Some("sql")                        => "🗄️ ",
        Some("lock")                       => "🔒",
        Some("log")                        => "📋",
        Some("exe") | Some("bin")          => "⚙️ ",
        None if path.is_dir()              => "📁",
        _                                  => "📄",
    }
}

fn format_size(bytes: u64) -> String {
    // We manually format here for clean output without external dep overhead
    if bytes < 1024 {
        format!("{}B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1}KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1}MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2}GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}