use crate::cli::Args;
use crate::display;
use crate::filters;
use crate::preview;
use crate::opener;

use colored::*;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use regex::Regex;
use walkdir::WalkDir;
use std::time::Instant;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::sync::{Arc, Mutex};
use std::thread;
use std::path::PathBuf;

#[derive(Debug)]
struct SearchResult {
    path: PathBuf,
    size: u64,
    modified: Option<String>,
    match_info: Option<String>,
    fuzzy_score: i64,
}

const SKIP_DIRS: &[&str] = &[
    ".git", "node_modules", "target", ".svn", ".hg",
    "__pycache__", ".next", "dist", "build", ".cache",
    "vendor", ".idea", ".vscode",
];

struct SearchConfig {
    name: Option<String>,
    fuzzy: bool,
    name_regex: Option<Regex>,
    content_query_lower: Option<String>,
    content_regex: Option<Regex>,
    ext: Vec<String>,
    files_only: bool,
    dirs_only: bool,
    size_min: Option<u64>,
    size_max: Option<u64>,
    after: Option<String>,
    before: Option<String>,
}

pub fn run_search(args: &Args) {
    let start = Instant::now();

    let size_min = args.size_min.as_deref().and_then(filters::parse_size);
    let size_max = args.size_max.as_deref().and_then(filters::parse_size);

    let name_regex: Option<Regex> = args.regex.as_ref().and_then(|r| {
        match Regex::new(r) {
            Ok(re) => Some(re),
            Err(e) => { display::print_error(&format!("Invalid regex: {}", e)); None }
        }
    });

    let content_regex: Option<Regex> = if args.content_regex {
        args.content.as_ref().and_then(|r| Regex::new(r).ok())
    } else {
        None
    };

    let config = Arc::new(SearchConfig {
        name: args.name.clone(),
        fuzzy: args.fuzzy,
        name_regex,
        content_query_lower: args.content.as_ref().map(|q| q.to_lowercase()),
        content_regex,
        ext: args.ext.clone(),
        files_only: args.files_only,
        dirs_only: args.dirs_only,
        size_min,
        size_max,
        after: args.after.clone(),
        before: args.before.clone(),
    });

    display::print_section("Searching", "🔍");

    let entries: Vec<_> = WalkDir::new(&args.dir)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| {
            if e.file_type().is_dir() {
                let name = e.file_name().to_str().unwrap_or("");
                return !SKIP_DIRS.contains(&name);
            }
            true
        })
        .filter_map(|e| e.ok())
        .collect();

    let total_scanned = entries.len();
    let entries = Arc::new(entries);

    let num_threads = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
        .min(8);

    let results: Arc<Mutex<Vec<SearchResult>>> = Arc::new(Mutex::new(Vec::new()));
    let chunk_size = (total_scanned / num_threads).max(1);
    let mut handles = Vec::new();

    for chunk_idx in 0..num_threads {
        let start_idx = chunk_idx * chunk_size;
        let end_idx = if chunk_idx == num_threads - 1 {
            total_scanned
        } else {
            (start_idx + chunk_size).min(total_scanned)
        };
        if start_idx >= total_scanned { break; }

        let config = Arc::clone(&config);
        let entries = Arc::clone(&entries);
        let results = Arc::clone(&results);

        let handle = thread::spawn(move || {
            let fuzzy_matcher = SkimMatcherV2::default();
            let mut local: Vec<SearchResult> = Vec::new();

            for entry in &entries[start_idx..end_idx] {
                let path = entry.path();
                let is_dir = path.is_dir();

                if config.files_only && is_dir { continue; }
                if config.dirs_only && !is_dir { continue; }

                let meta = match entry.metadata() {
                    Ok(m) => m, Err(_) => continue,
                };

                if !filters::passes_ext_filter(path, &config.ext) { continue; }
                if !filters::passes_size_filter(&meta, config.size_min, config.size_max) { continue; }
                if !filters::passes_date_filter(&meta, config.after.as_deref(), config.before.as_deref()) { continue; }

                let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

                let mut fuzzy_score: i64 = 0;
                let name_passes = if let Some(ref query) = config.name {
                    if config.fuzzy {
                        if let Some(score) = fuzzy_matcher.fuzzy_match(file_name, query) {
                            fuzzy_score = score; true
                        } else { false }
                    } else if let Some(ref re) = config.name_regex {
                        re.is_match(file_name)
                    } else {
                        glob_match(file_name, query)
                    }
                } else if let Some(ref re) = config.name_regex {
                    re.is_match(file_name)
                } else {
                    true
                };

                if !name_passes { continue; }

                let mut match_info: Option<String> = None;
                if let Some(ref query_lower) = config.content_query_lower {
                    if !is_dir {
                        if meta.len() > 10 * 1024 * 1024 { continue; }
                        match_info = search_file_content_buffered(
                            path, query_lower, config.content_regex.as_ref(),
                        );
                        if match_info.is_none() { continue; }
                    }
                }

                local.push(SearchResult {
                    path: path.to_path_buf(),
                    size: meta.len(),
                    modified: filters::format_modified(&meta),
                    match_info,
                    fuzzy_score,
                });
            }

            // One lock acquire per thread — not per file
            results.lock().unwrap().extend(local);
        });

        handles.push(handle);
    }

    for handle in handles { handle.join().unwrap(); }

    let mut results = Arc::try_unwrap(results).unwrap().into_inner().unwrap();

    match args.sort.as_str() {
        "size" => results.sort_by(|a, b| b.size.cmp(&a.size)),
        "date" => results.sort_by(|a, b| b.modified.cmp(&a.modified)),
        _ => {
            if args.fuzzy {
                results.sort_by(|a, b| b.fuzzy_score.cmp(&a.fuzzy_score));
            } else {
                results.sort_by(|a, b| a.path.cmp(&b.path));
            }
        }
    }

    let display_count = if args.limit > 0 { results.len().min(args.limit) } else { results.len() };

    if results.is_empty() {
        display::print_warning("No files matched your search.");
        if args.fuzzy { display::print_info("Try broadening your search term."); }
    } else {
        for (i, result) in results[..display_count].iter().enumerate() {
            if args.fuzzy && result.fuzzy_score > 0 {
                display::print_fuzzy_result(i + 1, &result.path, result.fuzzy_score, result.size);
            } else {
                display::print_result(i + 1, &result.path, result.size,
                    result.modified.clone(), result.match_info.clone());
            }
            if args.preview > 0 && result.path.is_file() {
                preview::show_preview(&result.path, args.preview);
            }
        }

        if display_count < results.len() {
            println!("  {} {} more result{} hidden. Use --limit 0 to see all.",
                "…".yellow(),
                (results.len() - display_count).to_string().yellow().bold(),
                if results.len() - display_count == 1 { "" } else { "s" });
        }

        if let Some(editor) = &args.open {
            if let Some(first) = results.first() {
                opener::open_with(&first.path, Some(editor));
            }
        } else if !results.is_empty() {
            println!("\n  {} {}", "💡".yellow(),
                "Tip: Use --open vim (or code/nano/subl) to open the first result".dimmed());
        }
    }

    display::print_summary(display_count, total_scanned, start.elapsed().as_millis());
}

fn search_file_content_buffered(
    path: &std::path::Path,
    query_lower: &str,
    regex: Option<&Regex>,
) -> Option<String> {
    let file = File::open(path).ok()?;
    let mut reader = BufReader::with_capacity(64 * 1024, file);

    // Peek at the first chunk — if null bytes exist, it's binary, skip it
    {
        let peek = reader.fill_buf().ok()?;
        if peek.iter().take(512).any(|&b| b == 0) { return None; }
    }

    let mut matches: Vec<String> = Vec::new();
    let mut line = String::new();
    let mut line_num = 0usize;

    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                line_num += 1;
                let found = if let Some(re) = regex {
                    re.is_match(&line)
                } else {
                    line.to_lowercase().contains(query_lower)
                };
                if found {
                    let trimmed = line.trim_end();
                    let snippet = if trimmed.len() > 80 {
                        format!("{}…", &trimmed[..80])
                    } else { trimmed.to_string() };
                    matches.push(format!("L{}: {}", line_num, snippet));
                    if matches.len() >= 3 { break; }
                }
            }
            Err(_) => break,
        }
    }

    if matches.is_empty() { None } else { Some(matches.join(" | ")) }
}

fn glob_match(filename: &str, pattern: &str) -> bool {
    let fl = filename.to_lowercase();
    let pl = pattern.to_lowercase();
    if pl.contains('*') {
        let parts: Vec<&str> = pl.split('*').collect();
        let mut pos = 0usize;
        for (i, part) in parts.iter().enumerate() {
            if part.is_empty() { continue; }
            if i == 0 {
                if !fl.starts_with(part) { return false; }
                pos = part.len();
            } else if let Some(found) = fl[pos..].find(part) {
                pos += found + part.len();
            } else { return false; }
        }
        if let Some(last) = parts.last() {
            if !last.is_empty() && !fl.ends_with(last) { return false; }
        }
        true
    } else {
        fl.contains(&pl)
    }
}
