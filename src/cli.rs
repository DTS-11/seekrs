use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "seekrs",
    author,
    version,
    about,
    long_about = None,
    after_help = "Tip: Combine flags for powerful searches! e.g. --ext rs --content \"TODO\" --fuzzy"
)]

pub struct Args {
    /// Root directory to search in (default: current dir)
    #[arg(short = 'd', long, default_value = ".")]
    pub dir: String,

    /// File name to search for (supports wildcards like *.rs)
    #[arg(short = 'n', long)]
    pub name: Option<String>,

    /// Enable fuzzy matching — finds close/misspelled names
    #[arg(short = 'z', long)]
    pub fuzzy: bool,

    /// Regex pattern to match against file names
    #[arg(short = 'r', long)]
    pub regex: Option<String>,

    /// Search inside file contents for this text
    #[arg(short = 'c', long)]
    pub content: Option<String>,

    /// Use regex when searching file contents
    #[arg(long)]
    pub content_regex: bool,

    /// Preview file contents interactively (first N lines)
    #[arg(short = 'p', long, default_value = "0")]
    pub preview: usize,

    /// Filter by file extension(s) e.g. --ext rs toml md
    #[arg(short = 'e', long, num_args = 1..)]
    pub ext: Vec<String>,

    /// Only show files (not directories)
    #[arg(long)]
    pub files_only: bool,

    /// Only show directories
    #[arg(long)]
    pub dirs_only: bool,

    /// Minimum file size e.g. 10KB, 1MB, 500B
    #[arg(long)]
    pub size_min: Option<String>,

    /// Maximum file size e.g. 100MB, 1GB
    #[arg(long)]
    pub size_max: Option<String>,

    /// Show files modified after this date (YYYY-MM-DD)
    #[arg(long)]
    pub after: Option<String>,

    /// Show files modified before this date (YYYY-MM-DD)
    #[arg(long)]
    pub before: Option<String>,

    /// Display a pretty directory tree
    #[arg(short = 't', long)]
    pub tree: bool,

    /// Detect duplicate files by content hash
    #[arg(long)]
    pub duplicates: bool,

    /// Open found file(s) with this editor (e.g. vim, code, nano)
    #[arg(short = 'o', long)]
    pub open: Option<String>,

    /// Max results to show (0 = unlimited)
    #[arg(long, default_value = "0")]
    pub limit: usize,

    /// Sort results by: name | size | date
    #[arg(long, default_value = "name")]
    pub sort: String,

    /// Upgrade to the latest version
    #[arg(long)]
    pub upgrade: bool,
}
