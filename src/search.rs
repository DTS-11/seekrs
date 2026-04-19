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

#[derive(Debug)]
struct SearchResult {
    path: std::path::PathBuf,
    size: u64,
    modified: Option<String>,
    match_info: Option<String>,
    fuzzy_score: i64,
}

pub fn run_search(args: &Args) {
    let start = Instant::now();

    let size_min = args.size_min.as_deref().and_then(filters::parse_size);
    let size_max = args.size_max.as_deref().and_then(filters::parse_size);

    // Compile regex if provided
    let name_regex: Option<Regex> = args.regex.as_ref().and_then(|r| {
        match Regex::new(r) {
            Ok(re) => Some(re),
            Err(e) => {
                display::print_error(&format!("Invalid regex: {}", e));
                None
            }
        }
    });

    let content_regex: Option<Regex> = if args.content_regex {
        args.content.as_ref().and_then(|r| Regex::new(r).ok())
    } else {
        None
    };

    // Set up the fuzzy matcher (Skim algorithm)
    let fuzzy_matcher = SkimMatcherV2::default();

    display::print_section("Searching", "🔍");

    let mut results: Vec<SearchResult> = Vec::new();
    let mut files_scanned: usize = 0;

    for entry in WalkDir::new(&args.dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        files_scanned += 1;

        let path = entry.path();
        let is_dir = path.is_dir();

        if args.files_only && is_dir { continue; }
        if args.dirs_only && !is_dir { continue; }

        let meta = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };

        if !filters::passes_ext_filter(path, &args.ext) { continue; }

        if !filters::passes_size_filter(&meta, size_min, size_max) { continue; }

        if !filters::passes_date_filter(
            &meta,
            args.after.as_deref(),
            args.before.as_deref()
        ) { continue; }

        let file_name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");

        let mut fuzzy_score: i64 = 0;
        let name_passes = if let Some(query) = &args.name {
            if args.fuzzy {
                // Fuzzy match: returns Some(score) if close enough
                if let Some(score) = fuzzy_matcher.fuzzy_match(file_name, query) {
                    fuzzy_score = score;
                    true
                } else {
                    false
                }
            } else if let Some(ref re) = name_regex {
                re.is_match(file_name)
            } else {
                // Simple case-insensitive substring match
                // Also support basic glob: if query has *, split on it
                glob_match(file_name, query)
            }
        } else if name_regex.is_some() {
            // Regex provided but no --name: match against filename
            name_regex.as_ref().unwrap().is_match(file_name)
        } else {
            true // No name filter = match everything
        };

        if !name_passes { continue; }

        let mut match_info: Option<String> = None;

        if let Some(query) = &args.content {
            if !is_dir {
                match_info = search_file_content(path, query, content_regex.as_ref());
                if match_info.is_none() {
                    continue; // content not found in this file
                }
            }
        }

        let modified = filters::format_modified(&meta);
        let size = meta.len();

        results.push(SearchResult {
            path: path.to_path_buf(),
            size,
            modified,
            match_info,
            fuzzy_score,
        });
    }

    match args.sort.as_str() {
        "size" => results.sort_by(|a, b| b.size.cmp(&a.size)),
        "date" => results.sort_by(|a, b| b.modified.cmp(&a.modified)),
        _      => {
            if args.fuzzy {
                // Sort fuzzy results by score descending (best match first)
                results.sort_by(|a, b| b.fuzzy_score.cmp(&a.fuzzy_score));
            } else {
                results.sort_by(|a, b| a.path.cmp(&b.path));
            }
        }
    }

    let display_count = if args.limit > 0 {
        results.len().min(args.limit)
    } else {
        results.len()
    };

    if results.is_empty() {
        display::print_warning("No files matched your search.");
        if args.fuzzy {
            display::print_info("Try broadening your search term.");
        }
    } else {
        for (i, result) in results[..display_count].iter().enumerate() {
            if args.fuzzy && result.fuzzy_score > 0 {
                display::print_fuzzy_result(i + 1, &result.path, result.fuzzy_score, result.size);
            } else {
                display::print_result(
                    i + 1,
                    &result.path,
                    result.size,
                    result.modified.clone(),
                    result.match_info.clone(),
                );
            }

            // Show content preview if --preview N is set
            if args.preview > 0 && result.path.is_file() {
                preview::show_preview(&result.path, args.preview);
            }
        }

        if display_count < results.len() {
            println!(
                "  {} {} more result{} hidden. Use --limit 0 to see all.",
                "…".yellow(),
                (results.len() - display_count).to_string().yellow().bold(),
                if results.len() - display_count == 1 { "" } else { "s" }
            );
        }

        // If --open is set, open the first result
        if let Some(editor) = &args.open {
            if let Some(first) = results.first() {
                opener::open_with(&first.path, Some(editor));
            }
        } else if args.open.is_none() && !results.is_empty() {
            println!(
                "\n  {} {}",
                "💡".yellow(),
                "Tip: Use --open vim (or code/nano/subl) to open the first result".dimmed()
            );
        }
    }

    let elapsed = start.elapsed().as_millis();
    display::print_summary(display_count, files_scanned, elapsed);
}

fn search_file_content(
    path: &std::path::Path,
    query: &str,
    regex: Option<&Regex>,
) -> Option<String> {
    // Only search text-ish files (skip binary/large files)
    let content = std::fs::read_to_string(path).ok()?;

    let mut matches: Vec<String> = Vec::new();

    for (line_num, line) in content.lines().enumerate() {
        let found = if let Some(re) = regex {
            re.is_match(line)
        } else {
            line.to_lowercase().contains(&query.to_lowercase())
        };

        if found {
            let snippet = if line.len() > 80 {
                format!("{}…", &line[..80])
            } else {
                line.to_string()
            };
            matches.push(format!("L{}: {}", line_num + 1, snippet));

            if matches.len() >= 3 { break; } // Show max 3 matches per file
        }
    }

    if matches.is_empty() {
        None
    } else {
        Some(matches.join(" | "))
    }
}

fn glob_match(filename: &str, pattern: &str) -> bool {
    let filename_lower = filename.to_lowercase();
    let pattern_lower = pattern.to_lowercase();

    if pattern_lower.contains('*') {
        // Split pattern on * and check each part is present in order
        let parts: Vec<&str> = pattern_lower.split('*').collect();
        let mut pos = 0usize;
        for (i, part) in parts.iter().enumerate() {
            if part.is_empty() { continue; }
            if i == 0 {
                if !filename_lower.starts_with(part) { return false; }
                pos = part.len();
            } else if let Some(found) = filename_lower[pos..].find(part) {
                pos += found + part.len();
            } else {
                return false;
            }
        }
        if let Some(last) = parts.last() {
            if !last.is_empty() && !filename_lower.ends_with(last) {
                return false;
            }
        }
        true
    } else {
        filename_lower.contains(&pattern_lower)
    }
}