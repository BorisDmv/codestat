use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use colored::*;

fn main() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let ignore_set = load_ignore_list(&current_dir);

    let mut language_stats: HashMap<String, (usize, usize)> = HashMap::new();
    let mut total_files = 0;

    for entry in WalkDir::new(&current_dir)
        .into_iter()
        .filter_entry(|e| !should_ignore(e.path(), &ignore_set))
        .filter_map(Result::ok)
        .filter(|e| e.path().is_file())
    {
        let path = entry.path();
        let extension = path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        match count_lines(path) {
            Ok((code, empty)) => {
                let stats = language_stats.entry(extension).or_insert((0, 0));
                stats.0 += code;
                stats.1 += empty;
                total_files += 1;
            }
            Err(_) => continue,
        }
    }

    println!("\n{}", "Code Statistics".bold().underline());
    println!("{} files scanned.\n", total_files);

    let mut grand_total = 0;
    let mut grand_code_total = 0;

    for (_ext, (code, empty)) in &language_stats {
        grand_total += code + empty;
        grand_code_total += code;
    }

    for (ext, (code, empty)) in &language_stats {
        let total = code + empty;
        let percent = if grand_code_total > 0 {
            (*code as f64 / grand_code_total as f64) * 100.0
        } else {
            0.0
        };
        println!(
            "{}: {} code lines, {} empty lines ({} total) - {:.2}%",
            ext.green(),
            code.to_string().yellow(),
            empty.to_string().blue(),
            total.to_string().white(),
            percent.to_string().magenta()
        );
    }

    println!("\n{}: {} lines total", "Grand Total".bold(), grand_total.to_string().cyan());
    println!("{}: {} lines of code (excluding empty lines)", "Code Only".bold(), grand_code_total.to_string().cyan());
}

fn count_lines(path: &Path) -> io::Result<(usize, usize)> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut code_lines = 0;
    let mut empty_lines = 0;

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            empty_lines += 1;
        } else {
            code_lines += 1;
        }
    }

    Ok((code_lines, empty_lines))
}

fn load_ignore_list(root: &Path) -> HashSet<String> {
    let mut ignore_set = HashSet::new();
    let ignore_path = root.join(".codestatignore");

    if let Ok(contents) = fs::read_to_string(ignore_path) {
        for line in contents.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with('#') {
                ignore_set.insert(trimmed.to_string());
            }
        }
    } else {
        // Default ignore list
        for default in ["node_modules", ".git", "dist", "target"] {
            ignore_set.insert(default.to_string());
        }
    }

    ignore_set
}

fn should_ignore(path: &Path, ignore_set: &HashSet<String>) -> bool {
    // Check if the file name starts with a dot
    if let Some(file_name) = path.file_name() {
        if let Some(name_str) = file_name.to_str() {
            if name_str.starts_with('.') {
                return true;
            }
        }
    }

    // Check against ignore list
    for component in path.components() {
        if let std::path::Component::Normal(name) = component {
            if let Some(name_str) = name.to_str() {
                if ignore_set.contains(name_str) {
                    return true;
                }
            }
        }
    }
    false
}