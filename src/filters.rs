use std::fs::Metadata;
use chrono::{DateTime, Local, NaiveDate};

pub fn parse_size(s: &str) -> Option<u64> {
    // Try to split the numeric part from the unit part
    // e.g. "10KB" → num_str="10", unit="KB"
    let s = s.trim().to_uppercase();

    // Find where digits end
    let split_pos = s.find(|c: char| !c.is_ascii_digit() && c != '.')?;
    let num_str = &s[..split_pos];
    let unit = s[split_pos..].trim();

    // Parse the number — `?` propagates None upward if parsing fails
    let num: f64 = num_str.parse().ok()?;

    // Match the unit string to a multiplier
    let multiplier: u64 = match unit {
        "B" | ""   => 1,
        "KB" | "K" => 1024,
        "MB" | "M" => 1024 * 1024,
        "GB" | "G" => 1024 * 1024 * 1024,
        _          => return None,
    };

    // `as u64` casts f64 → u64 (truncates decimals)
    Some((num * multiplier as f64) as u64)
}

pub fn passes_size_filter(
    meta: &Metadata,
    size_min: Option<u64>,
    size_max: Option<u64>,
) -> bool {
    let file_size = meta.len(); // file size in bytes

    if let Some(min) = size_min {
        if file_size < min {
            return false; // file too small
        }
    }

    if let Some(max) = size_max {
        if file_size > max {
            return false; // file too large
        }
    }

    true
}

pub fn passes_date_filter(
    meta: &Metadata,
    after: Option<&str>,
    before: Option<&str>,
) -> bool {
    // Get the modification time from OS metadata
    let modified = match meta.modified().ok() {
        Some(t) => t,
        None    => return true, // can't read date, skip filter
    };

    // Convert SystemTime → DateTime<Local> for comparison
    let modified_dt: DateTime<Local> = modified.into();
    let modified_date = modified_dt.date_naive();

    if let Some(after_str) = after {
        if let Ok(after_date) = NaiveDate::parse_from_str(after_str, "%Y-%m-%d") {
            if modified_date < after_date {
                return false;
            }
        }
    }

    if let Some(before_str) = before {
        if let Ok(before_date) = NaiveDate::parse_from_str(before_str, "%Y-%m-%d") {
            if modified_date > before_date {
                return false;
            }
        }
    }

    true
}

pub fn passes_ext_filter(path: &std::path::Path, extensions: &[String]) -> bool {
    if extensions.is_empty() {
        return true;
    }

    // Get the file's extension, compare case-insensitively
    match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => extensions
            .iter()
            .any(|f| f.eq_ignore_ascii_case(ext)),
        None => false, // no extension and filter is active → skip
    }
}

pub fn format_modified(meta: &Metadata) -> Option<String> {
    let modified = meta.modified().ok()?;
    let dt: DateTime<Local> = modified.into();
    Some(dt.format("%Y-%m-%d %H:%M").to_string())
}
