<div align="center">

```
███████╗███████╗███████╗██╗  ██╗██████╗ ███████╗
██╔════╝██╔════╝██╔════╝██║ ██╔╝██╔══██╗██╔════╝
███████╗█████╗  █████╗  █████╔╝ ██████╔╝███████╗
╚════██║██╔══╝  ██╔══╝  ██╔═██╗ ██╔══██╗╚════██║
███████║███████╗███████╗██║  ██╗██║  ██║███████║
╚══════╝╚══════╝╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝
```

**A blazing-fast, colorful file search tool — built in Rust.**

[![Rust](https://img.shields.io/badge/built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/seekrs?style=flat-square)](https://crates.io/crates/seekrs)
[![GitHub Release](https://img.shields.io/github/v/release/yourusername/seekrs?style=flat-square)](https://github.com/yourusername/seekrs/releases)

[Installation](#installation) · [Features](#features) · [Usage](#usage) · [Examples](#examples) · [Contributing](#contributing)

---

## What is seekrs?

**seekrs** is a command-line file search tool that goes far beyond basic `find` or `ls`. It combines fuzzy matching, content search, regex, duplicate detection, and a beautiful directory tree — all with colorized, emoji-rich terminal output.

Think of it as `find` + `grep` + `fzf` + `tree`, rolled into one fast Rust binary.

---

## Features

| Feature | Flag | Description |
|---|---|---|
| 🔤 Name search | `--name` | Search by filename with wildcard support (`*.rs`) |
| 🌀 Fuzzy matching | `--fuzzy` | Finds close matches even with typos or misspellings |
| 📜 Regex search | `--regex` | Match filenames against a regular expression |
| 🔍 Content search | `--content` | Search inside file contents (like grep) |
| 🧩 Content regex | `--content-regex` | Use regex when searching inside files |
| 👁️ File preview | `--preview` | Show the first N lines of matched files inline |
| 📂 Type filter | `--ext` | Filter by one or more file extensions |
| 📌 Files only | `--files-only` | Exclude directories from results |
| 📁 Dirs only | `--dirs-only` | Show only directories |
| 📦 Size filter | `--size-min` / `--size-max` | Filter by file size (e.g. `10KB`, `1MB`) |
| 📅 Date filter | `--after` / `--before` | Filter by last modified date |
| 🌳 Directory tree | `--tree` | Print a beautiful Unicode directory tree |
| 👯 Duplicate finder | `--duplicates` | Detect identical files by content hash (MD5) |
| ✏️ Open in editor | `--open` | Open the first result in any editor |
| 📊 Sort results | `--sort` | Sort by `name`, `size`, or `date` |
| 🔢 Limit output | `--limit` | Cap the number of results shown |

---

## Installation

### Option 1 — Download a pre-built binary (no Rust required)

Go to the [Releases page](https://github.com/yourusername/seekrs/releases) and download the binary for your platform:

| Platform | File |
|---|---|
| Linux (x86_64) | `seekrs-x86_64-unknown-linux-gnu.tar.gz` |
| macOS (Intel) | `seekrs-x86_64-apple-darwin.tar.gz` |
| macOS (Apple Silicon) | `seekrs-aarch64-apple-darwin.tar.gz` |
| Windows | `seekrs-x86_64-pc-windows-msvc.zip` |

**Linux / macOS:**
```bash
tar -xzf seekrs-*.tar.gz
chmod +x seekrs
sudo mv seekrs /usr/local/bin/
```

**Windows:** Extract the `.zip` and run `seekrs.exe` from PowerShell or Command Prompt.

---

### Option 2 — Install via Cargo (requires Rust)

```bash
cargo install seekrs
```

---

### Option 3 — Build from source

```bash
git clone https://github.com/DTS-11/seekrs
cd seekrs
cargo build --release
./target/release/seekrs --help
```

---

## Usage

```
seekrs [OPTIONS]
```

### All flags

```
  -d, --dir <DIR>            Root directory to search in [default: .]
  -n, --name <NAME>          File name to search for (supports wildcards)
  -z, --fuzzy                Enable fuzzy matching for typos/misspellings
  -r, --regex <REGEX>        Regex pattern to match against file names
  -c, --content <TEXT>       Search inside file contents
      --content-regex        Use regex when searching file contents
  -p, --preview <N>          Preview first N lines of matched files
  -e, --ext <EXT>...         Filter by file extension(s)
      --files-only           Only show files
      --dirs-only            Only show directories
      --size-min <SIZE>      Minimum file size (e.g. 10KB, 1MB)
      --size-max <SIZE>      Maximum file size (e.g. 100MB, 1GB)
      --after <YYYY-MM-DD>   Files modified after this date
      --before <YYYY-MM-DD>  Files modified before this date
  -t, --tree                 Display a directory tree
      --duplicates           Detect duplicate files by content hash
  -o, --open <EDITOR>        Open first result in an editor
      --sort <FIELD>         Sort by: name | size | date [default: name]
      --limit <N>            Max results to show (0 = unlimited)
  -h, --help                 Print help
  -V, --version              Print version
```

---

## Examples

### Basic name search
```bash
seekrs --name "config"
```

### Fuzzy search — finds results even with typos
```bash
seekrs --name "mian" --fuzzy
# → finds main.rs, main.py, etc.
```

### Search inside file contents
```bash
seekrs --content "TODO" --ext rs
```

### Regex filename matching
```bash
seekrs --regex "^test_.*\.py$"
```

### Content search with regex
```bash
seekrs --content "async fn \w+" --content-regex --ext rs
```

### Filter by file size
```bash
seekrs --size-min 1MB --size-max 50MB
```

### Filter by date
```bash
seekrs --after 2025-01-01 --before 2025-12-31
```

### Preview file contents inline
```bash
seekrs --name "README" --preview 10
```

### Pretty directory tree (Rust files only)
```bash
seekrs --tree --ext rs
```

### Find duplicate files
```bash
seekrs --duplicates --dir ~/Downloads
```

### Open result in an editor
```bash
seekrs --name "main.rs" --open vim
seekrs --name "main.rs" --open code
seekrs --name "main.rs" --open nano
```

### Sort and limit results
```bash
seekrs --ext log --sort size --limit 20
```

### Combine multiple flags for powerful searches
```bash
seekrs --name "server" --fuzzy --ext rs --content "async fn" --sort size
seekrs --dir ~/projects --ext ts --content "useEffect" --after 2025-06-01
seekrs --size-min 10MB --sort size --files-only --limit 10
```

---

## Output

seekrs produces colorized, emoji-rich output designed to be fast to scan:

```
╔══════════════════════════════════════════════════════╗
║                                                      ║
║   🔍  S E E K R S  —  v0.1.0                       ║
║     blazing fast · colorful · feature-rich           ║
╚══════════════════════════════════════════════════════╝

  🔍 Searching
  ────────────────────────────────────────────────
    1.  🦀 ./src/main.rs  [791B]  📅 2025-04-18 12:32
    2.  🦀 ./src/search.rs  [8.4KB]  📅 2025-04-18 12:32
         ↳ L41: pub fn run_search(args: &Args) {

  ✅ Found 2 results in 312 files  ⏱  94ms
```

File type icons are automatically assigned — 🦀 for Rust, 🐍 for Python, ⚡ for JS/TS, 🌐 for HTML, 🎨 for CSS, 📝 for Markdown, and more.

---

## Performance

seekrs is built in Rust and compiled to a native binary with no runtime overhead. On typical project directories (thousands of files), searches complete in under 200ms. Content search scales linearly with file sizes — binary and very large files (>50MB) are skipped automatically.

---

## Supported Platforms

- ✅ Linux (x86_64)
- ✅ macOS (Intel + Apple Silicon)
- ✅ Windows 10/11

---

## Built With

| Crate | Purpose |
|---|---|
| [clap](https://crates.io/crates/clap) | CLI argument parsing |
| [colored](https://crates.io/crates/colored) | Terminal colors and styling |
| [fuzzy-matcher](https://crates.io/crates/fuzzy-matcher) | Fuzzy matching (Skim algorithm) |
| [walkdir](https://crates.io/crates/walkdir) | Recursive directory traversal |
| [regex](https://crates.io/crates/regex) | Regular expression engine |
| [chrono](https://crates.io/crates/chrono) | Date parsing and comparison |
| [md5](https://crates.io/crates/md5) | File hashing for duplicate detection |
| [open](https://crates.io/crates/open) | Cross-platform file opening |

---

## Contributing

Contributions are welcome! To get started:

```bash
git clone https://github.com/DTS-11/seekrs
cd seekrs
cargo build
cargo test
```

Please open an issue before submitting a large pull request so we can discuss the approach first.

---

## License

MIT © [Deon](https://github.com/DTS-11)

---

<div align="center">
  <sub>If seekrs saved you time, consider giving it a ⭐ on GitHub.</sub>
</div>