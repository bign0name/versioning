use std::process::Command;
use std::str;
use arboard::Clipboard;
use regex::Regex;

fn main() {
    match get_latest_version() {
        Ok((version, prefix)) => {
            println!("{}", version);
            if let Err(e) = copy_to_clipboard(&prefix) {
                eprintln!("Failed to copy to clipboard: {}", e);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn get_latest_version() -> Result<(String, String), String> {
    // Run `git tag` to get all tags
    let output = Command::new("git")
        .arg("tag")
        .output()
        .map_err(|e| format!("Failed to run git tag: {}", e))?;

    if !output.status.success() {
        return Err("git tag command failed".to_string());
    }

    let tags = str::from_utf8(&output.stdout)
        .map_err(|e| format!("Invalid UTF-8 in git tag output: {}", e))?;

    // Regex for version format: YEAR[VARIANT].MAJOR.MINOR.PATCH
    let re = Regex::new(r"^(\d{4})([a-z])?\.(\d+)\.(\d+)\.(\d+)$")
        .map_err(|e| format!("Invalid regex: {}", e))?;

    // Parse and sort versions
    let mut versions: Vec<(u32, Option<char>, u32, u32, u32, String)> = tags
        .lines()
        .filter_map(|tag| {
            if let Some(caps) = re.captures(tag.trim()) {
                let year: u32 = caps[1].parse().ok()?;
                let variant = caps.get(2).map(|m| m.as_str().chars().next().unwrap());
                let major: u32 = caps[3].parse().ok()?;
                let minor: u32 = caps[4].parse().ok()?;
                let patch: u32 = caps[5].parse().ok()?;
                let full_version = tag.trim().to_string();
                Some((year, variant, major, minor, patch, full_version))
            } else {
                None
            }
        })
        .collect();

    if versions.is_empty() {
        return Err("No valid version tags found".to_string());
    }

    // Sort by year, variant (None first, then a, b, ...), major, minor, patch
    versions.sort_by(|a, b| {
        a.0.cmp(&b.0) // Year
            .then(a.1.unwrap_or('\0').cmp(&b.1.unwrap_or('\0'))) // Variant (None as \0)
            .then(a.2.cmp(&b.2)) // Major
            .then(a.3.cmp(&b.3)) // Minor
            .then(a.4.cmp(&b.4)) // Patch
    });

    // Get the latest version
    let latest = versions.last().unwrap();
    let prefix = match latest.1 {
        Some(v) => format!("{}.", latest.0.to_string() + &v.to_string()),
        None => format!("{}.", latest.0),
    };

    Ok((latest.5.clone(), prefix))
}

fn copy_to_clipboard(text: &str) -> Result<(), String> {
    let mut clipboard = Clipboard::new()
        .map_err(|e| format!("Failed to initialize clipboard: {}", e))?;
    clipboard
        .set_text(text)
        .map_err(|e| format!("Failed to copy to clipboard: {}", e))?;
    Ok(())
}
