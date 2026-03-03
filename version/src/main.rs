use std::env;
use std::process::Command;
use std::str;
use arboard::Clipboard;
use regex::Regex;

#[derive(Debug)]
struct Version {
    year: u32,
    variant: Option<char>,
    phase: char,
    major: u32,
    minor: u32,
    patch: u32,
    suffix: Option<char>,
    full_version: String,
    prefix: String,
    date: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && (args[1] == "help" || args[1] == "--help" || args[1] == "-h") {
        print_help();
        return;
    }

    match get_latest_version() {
        Ok((version, prefix)) => {
            // Print version in bright yellow (\x1b[93m) and reset color (\x1b[0m)
            if let Some(prefix) = prefix {
                println!("\x1b[93m{}\x1b[0m - Copied: \"{}\"", version, prefix);
                if let Err(e) = copy_to_clipboard(&prefix) {
                    eprintln!("Failed to copy to clipboard: {}", e);
                }
            } else {
                println!("\x1b[93m{}\x1b[0m", version);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn print_help() {
    println!(
"\x1b[1mYear-Phase Versioning with Variant and Stability Suffixes\x1b[0m

  Format: YEAR[VARIANT].PHASE.MAJOR.MINOR.PATCH[-b|-p]

  \x1b[1mYEAR\x1b[0m       Release year (e.g., 2026), omitted for internal phase, starts in June (flexible guideline).
  \x1b[1mVARIANT\x1b[0m    Optional letter (a, b, c, ...) for distinct release streams in a year.
  \x1b[1mPHASE\x1b[0m      i (internal), b (beta), p (public beta), s (stable).
  \x1b[1mMAJOR\x1b[0m      Breaking changes since last release (0 means none). Resets MINOR and PATCH to 0.
  \x1b[1mMINOR\x1b[0m      New features or improvements. Resets PATCH to 0.
  \x1b[1mPATCH\x1b[0m      Bug fixes or minor updates.
  \x1b[1mSuffix\x1b[0m     Optional -b (beta) or -p (public beta) for non-stable releases.

  \x1b[1mPhase Rules\x1b[0m
  - Internal phase: Omits YEAR, uses PHASE=i, no -b or -p (e.g., i.0.32.102).
  - Other phases: Include YEAR, use PHASE=b, p, or s.
  - Beta phase omits -b/-p. Public beta uses -b (not -p). Stable uses -b or -p.
  - Can skip beta or public beta, but beta testing is recommended.
  - Version resets to 0.0.0 when entering a new phase or variant.
  - A commit can have multiple versions to mark a beta's transition to stable.

  \x1b[1mExamples\x1b[0m
  i.0.32.102         Internal, no breaking changes, 32 minor, 102 patches.
  2026.b.0.1.4       Beta, no breaking changes, 1 minor, 4 patches.
  2026.p.1.12.4-b    Public beta, 1 breaking, 12 minor, 4 patches, beta status.
  2026.s.2.3.4       Stable, 2 breaking, 3 minor, 4 patches.
  2026a.s.0.0.0-p    Stable, variant stream, public beta status.

  \x1b[1mNotes\x1b[0m
  - Use lightweight git tags for versioning (e.g., git tag v2026.s.1.0.0)." 
    );
}

fn get_latest_version() -> Result<(String, Option<String>), String> {
    // Run `git tag --list` with `--format` to get tag and date
    let output = Command::new("git")
        .args(["tag", "--list", "--format=%(refname:short):%(creatordate:iso-strict)"])
        .output()
        .map_err(|e| format!("Failed to run git tag: {}", e))?;

    if !output.status.success() {
        return Err("git tag command failed".to_string());
    }

    let tags = str::from_utf8(&output.stdout)
        .map_err(|e| format!("Invalid UTF-8 in git tag output: {}", e))?;

    // Regex for phase-based versioning: vi.MAJOR.MINOR.PATCH or vYEAR[VARIANT].PHASE.MAJOR.MINOR.PATCH[-b|-p]
    let phase_re = Regex::new(
        r"^v((\d{4})([a-z])?\.)?([ibps])\.(\d+)\.(\d+)\.(\d+)(-b|-p)?$"
    ).map_err(|e| format!("Invalid phase regex: {}", e))?;

    let mut phase_versions: Vec<Version> = tags
        .lines()
        .filter_map(|line| {
            let (tag, date) = match line.split_once(':') {
                Some((t, d)) => (t, d),
                None => {
                    eprintln!("Debug: Invalid line format: {}", line);
                    return None;
                }
            };
            if let Some(caps) = phase_re.captures(tag) {
                let year = caps.get(2).map_or(0, |m| m.as_str().parse().unwrap_or(0));
                let variant = caps.get(3).map(|m| m.as_str().chars().next().unwrap());
                let phase = caps[4].chars().next().unwrap();
                let major: u32 = caps[5].parse().ok()?;
                let minor: u32 = caps[6].parse().ok()?;
                let patch: u32 = caps[7].parse().ok()?;
                let suffix = caps.get(8).map(|m| m.as_str().chars().next().unwrap());
                let full_version = tag.to_string();
                let prefix = if phase == 'i' {
                    "vi.".to_string()
                } else {
                    match variant {
                        Some(v) => format!("v{}.{}.", year.to_string() + &v.to_string(), phase),
                        None => format!("v{}.{}.", year, phase),
                    }
                };
                Some(Version {
                    year,
                    variant,
                    phase,
                    major,
                    minor,
                    patch,
                    suffix,
                    full_version,
                    prefix,
                    date: date.to_string(),
                })
            } else {
                eprintln!("Debug: Tag did not match regex: {}", tag);
                None
            }
        })
        .collect();

    if !phase_versions.is_empty() {
        // Sort by year, variant (None first), phase (s > p > b > i), major, minor, patch, suffix (None > p > b), date
        phase_versions.sort_by(|a, b| {
            a.year.cmp(&b.year)
                .then(a.variant.unwrap_or('\0').cmp(&b.variant.unwrap_or('\0')))
                .then(b.phase.cmp(&a.phase))
                .then(a.major.cmp(&b.major))
                .then(a.minor.cmp(&b.minor))
                .then(a.patch.cmp(&b.patch))
                .then(b.suffix.unwrap_or('\0').cmp(&a.suffix.unwrap_or('\0')))
                .then(a.date.cmp(&b.date))
        });

        let latest = phase_versions.last().unwrap();
        return Ok((latest.full_version.clone(), Some(latest.prefix.clone())));
    }

    // Fallback to semantic versioning: vMAJOR.MINOR.PATCH
    let semver_re = Regex::new(r"^v(\d+)\.(\d+)\.(\d+)$")
        .map_err(|e| format!("Invalid semver regex: {}", e))?;

    let mut semver_versions: Vec<(u32, u32, u32, String, String)> = tags
        .lines()
        .filter_map(|line| {
            let (tag, date) = match line.split_once(':') {
                Some((t, d)) => (t, d),
                None => {
                    eprintln!("Debug: Invalid semver line format: {}", line);
                    return None;
                }
            };
            if let Some(caps) = semver_re.captures(tag) {
                let major: u32 = caps[1].parse().ok()?;
                let minor: u32 = caps[2].parse().ok()?;
                let patch: u32 = caps[3].parse().ok()?;
                let full_version = tag.to_string();
                Some((major, minor, patch, full_version, date.to_string()))
            } else {
                eprintln!("Debug: Semver tag did not match regex: {}", tag);
                None
            }
        })
        .collect();

    if semver_versions.is_empty() {
        return Err("No valid version tags found".to_string());
    }

    // Sort semantic versions: major, minor, patch, date
    semver_versions.sort_by(|a, b| {
        a.0.cmp(&b.0)
            .then(a.1.cmp(&b.1))
            .then(a.2.cmp(&b.2))
            .then(a.4.cmp(&b.4))
    });

    let latest = semver_versions.last().unwrap();
    Ok((latest.3.clone(), None))
}

fn copy_to_clipboard(text: &str) -> Result<(), String> {
    let mut clipboard = Clipboard::new()
        .map_err(|e| format!("Failed to initialize clipboard: {}", e))?;
    clipboard
        .set_text(text)
        .map_err(|e| format!("Failed to copy to clipboard: {}", e))?;
    Ok(())
}
