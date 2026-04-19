use colored::*;
use std::path::Path;
use std::process::Command;

pub fn open_with(path: &Path, editor: Option<&str>) {
    match editor {
        Some(ed) => open_in_editor(path, ed),
        None     => open_default(path),
    }
}

fn open_in_editor(path: &Path, editor: &str) {
    println!(
        "\n  {} Opening {} in {}…",
        "✏️ ".cyan(),
        path.file_name().unwrap_or_default().to_string_lossy().white().bold(),
        editor.yellow().bold()
    );

    let status = Command::new(editor)
        .arg(path)
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("  {} Editor closed.", "✅".green());
        }
        Ok(s) => {
            println!(
                "  {} Editor exited with code: {}",
                "⚠️ ".yellow(),
                s.code().unwrap_or(-1).to_string().yellow()
            );
        }
        Err(e) => {
            println!(
                "  {} Could not open editor '{}': {}",
                "❌".red(),
                editor.red().bold(),
                e.to_string().dimmed()
            );
            println!(
                "  {} Make sure '{}' is installed and in your PATH.",
                "💡".yellow(),
                editor
            );
        }
    }
}

fn open_default(path: &Path) {
    println!(
        "\n  {} Opening {} with default app…",
        "🚀".cyan(),
        path.file_name().unwrap_or_default().to_string_lossy().white().bold()
    );

    match open::that(path) {
        Ok(_) => println!("  {} Opened successfully.", "✅".green()),
        Err(e) => println!(
            "  {} Could not open file: {}",
            "❌".red(),
            e.to_string().dimmed()
        ),
    }
}
