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
[![GitHub Release](https://img.shields.io/github/v/release/DTS-11/seekrs?style=flat-square)](https://github.com/DTS-11/seekrs/releases)

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

### Windows

1. Go to the [Releases page](https://github.com/DTS-11/seekrs/releases)
2. Download `seekrs-x86_64-pc-windows-msvc.zip`
3. Extract the `.zip` — you'll get `seekrs.exe`
4. Open **PowerShell** or **Command Prompt** in the folder where `seekrs.exe` is:
   - Hold `Shift` + `Right-click` inside the folder
   - Select **"Open PowerShell window here"**
5. Run it:

```powershell
.\seekrs.exe --help
.\seekrs.exe --name "main"
.\seekrs.exe --tree
```

> **To use `seekrs` from any folder (recommended):**
> 1. Move `seekrs.exe` to a permanent location e.g. `C:\Tools\`
> 2. Search **"Environment Variables"** in the Start menu
> 3. Click **"Edit the system environment variables"** → **Environment Variables**
> 4. Under **System variables**, select `Path` → **Edit** → **New**
> 5. Add `C:\Tools\` → click OK
> 6. Restart your terminal — now you can run `seekrs` from anywhere

---

### macOS

```bash
# Download and extract
curl -L https://github.com/DTS-11/seekrs/releases/latest/download/seekrs-aarch64-apple-darwin.tar.gz | tar -xz

# Make executable and move to PATH
chmod +x seekrs
sudo mv seekrs /usr/local/bin/

# Run
seekrs --help
```

> Use `seekrs-x86_64-apple-darwin.tar.gz` if you have an older Intel Mac.

> **Note:** On first run macOS may block it. Go to **System Settings → Privacy & Security** and click **"Allow Anyway"**.

---

### Linux

```bash
# Download and extract
curl -L https://github.com/DTS-11/seekrs/releases/latest/download/seekrs-x86_64-unknown-linux-gnu.tar.gz | tar -xz

# Make executable and move to PATH
chmod +x seekrs
sudo mv seekrs /usr/local/bin/

# Run
seekrs --help
```

---

### Via Cargo (requires Rust)

```bash
cargo install seekrs
```

---

### Build from source

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

> **Windows users:** prefix with `.\seekrs.exe` if seekrs isn't in your PATH yet, or just `seekrs` if you've added it to your PATH.

---

## Examples

All examples below work on Linux and macOS as-is. On Windows, replace `seekrs` with `.\seekrs.exe` (or just `seekrs` if it's in your PATH), and replace paths like `~/Downloads` with `C:\Users\YourName\Downloads`.

### Basic name search
```bash
# Linux / macOS
seekrs --name "config"

# Windows (before adding to PATH)
.\seekrs.exe --name "config"

# Windows (after adding to PATH)
seekrs --name "config"
```

### Search a specific directory
```bash
# Linux / macOS
seekrs --dir ~/projects --name "main"

# Windows
seekrs --dir "C:\Users\YourName\projects" --name "main"
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
# Linux / macOS
seekrs --duplicates --dir ~/Downloads

# Windows
seekrs --duplicates --dir "C:\Users\YourName\Downloads"
```

### Open result in an editor
```bash
# Linux / macOS
seekrs --name "main.rs" --open vim
seekrs --name "main.rs" --open nano

# Windows — use notepad, or any editor in your PATH
seekrs --name "main.rs" --open notepad
seekrs --name "main.rs" --open code        # VS Code
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