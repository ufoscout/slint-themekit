//! Keeps design values out of components.
//!
//! A UI is only themeable if its components never hard-code a color, a size
//! or a duration. [`scan_source`] finds such literals in Slint source and
//! [`scan_dir`] applies it to a whole UI tree, so the rule can be enforced by
//! an ordinary `cargo test`:
//!
//! ```no_run
//! use slint_themekit::lint;
//!
//! // Theme definitions are the one place where literals belong.
//! let findings = lint::scan_dir("ui".as_ref(), &["themes"])?;
//! assert!(findings.is_empty(), "{}", lint::report(&findings));
//! # Ok::<(), std::io::Error>(())
//! ```
//!
//! The scan is lexical: it flags color literals (`#3366ff`, `rgb(..)`,
//! `Colors.red`) and non-zero lengths and durations (`12px`, `250ms`).
//! Comments and string literals are ignored.

use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const UNITS: &[&str] = &["px", "phx", "pt", "in", "mm", "cm", "rem", "ms", "s"];
const COLOR_FUNCTIONS: &[&str] = &["rgb", "rgba", "hsv", "oklch"];

/// A hard-coded design value found in Slint source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Literal {
    /// 1-based line number.
    pub line: usize,
    /// 1-based column, in characters.
    pub column: usize,
    /// The offending source text, e.g. `#ff0000` or `12px`.
    pub text: String,
}

/// A [`Literal`] together with the file it was found in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    /// The `.slint` file.
    pub path: PathBuf,
    /// What was found and where.
    pub literal: Literal,
}

impl fmt::Display for Finding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            path,
            literal: Literal { line, column, text },
        } = self;
        write!(f, "{}:{line}:{column}: hard-coded `{text}`", path.display())
    }
}

/// Formats findings one per line, for assertion messages.
#[must_use]
pub fn report(findings: &[Finding]) -> String {
    findings
        .iter()
        .map(Finding::to_string)
        .collect::<Vec<_>>()
        .join("\n")
}

/// Returns every hard-coded design value in `source`.
#[must_use]
pub fn scan_source(source: &str) -> Vec<Literal> {
    let bytes = source.as_bytes();
    let mut spans = Vec::new();
    let mut i = 0;
    while let Some(&byte) = bytes.get(i) {
        let next = bytes.get(i + 1).copied();
        i = match byte {
            b'/' if next == Some(b'/') => find(bytes, i, b"\n").unwrap_or(bytes.len()),
            b'/' if next == Some(b'*') => {
                find(bytes, i + 2, b"*/").map_or(bytes.len(), |end| end + 2)
            }
            b'"' => skip_string(bytes, i + 1),
            b'#' => {
                let end = take_while(bytes, i + 1, |b| b.is_ascii_hexdigit());
                if end > i + 1 {
                    spans.push((i, end));
                }
                end.max(i + 1)
            }
            b if b.is_ascii_alphabetic() || b == b'_' => {
                let end = take_while(bytes, i, is_ident_byte);
                let word = &source[i..end];
                if COLOR_FUNCTIONS.contains(&word) && bytes.get(end) == Some(&b'(') {
                    spans.push((i, end));
                } else if word == "Colors" && bytes.get(end) == Some(&b'.') {
                    spans.push((i, take_while(bytes, end + 1, is_ident_byte)));
                }
                end
            }
            b if b.is_ascii_digit() => {
                let number_end = take_while(bytes, i, |b| b.is_ascii_digit() || b == b'.');
                let unit_end = take_while(bytes, number_end, |b| b.is_ascii_alphabetic());
                let is_zero = source[i..number_end]
                    .bytes()
                    .all(|b| b == b'0' || b == b'.');
                if UNITS.contains(&&source[number_end..unit_end]) && !is_zero {
                    spans.push((i, unit_end));
                }
                unit_end
            }
            _ => i + 1,
        };
    }
    spans
        .into_iter()
        .map(|(start, end)| to_literal(source, start, end))
        .collect()
}

/// Scans every `.slint` file below `dir`, recursively.
///
/// Paths in `exempt` are relative to `dir` and may name a file or a whole
/// directory; use them for the files that *define* themes.
///
/// # Errors
///
/// Any I/O error raised while reading the tree.
pub fn scan_dir(dir: &Path, exempt: &[&str]) -> io::Result<Vec<Finding>> {
    let mut findings = Vec::new();
    visit(dir, dir, exempt, &mut findings)?;
    Ok(findings)
}

fn visit(root: &Path, dir: &Path, exempt: &[&str], findings: &mut Vec<Finding>) -> io::Result<()> {
    let entries = {
        let mut entries = fs::read_dir(dir)?
            .map(|entry| entry.map(|e| e.path()))
            .collect::<io::Result<Vec<_>>>()?;
        entries.sort();
        entries
    };
    for path in entries {
        let relative = path.strip_prefix(root).unwrap_or(&path);
        if exempt.iter().any(|skip| relative.starts_with(skip)) {
            continue;
        }
        if path.is_dir() {
            visit(root, &path, exempt, findings)?;
        } else if path.extension().is_some_and(|ext| ext == "slint") {
            let source = fs::read_to_string(&path)?;
            findings.extend(scan_source(&source).into_iter().map(|literal| Finding {
                path: path.clone(),
                literal,
            }));
        }
    }
    Ok(())
}

fn is_ident_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-'
}

fn take_while(bytes: &[u8], start: usize, keep: impl Fn(u8) -> bool) -> usize {
    bytes
        .get(start..)
        .and_then(|rest| rest.iter().position(|&b| !keep(b)))
        .map_or(bytes.len(), |offset| start + offset)
}

fn find(bytes: &[u8], start: usize, needle: &[u8]) -> Option<usize> {
    bytes
        .get(start..)?
        .windows(needle.len())
        .position(|window| window == needle)
        .map(|offset| start + offset)
}

/// Returns the index just past the closing quote of a string starting at `start`.
fn skip_string(bytes: &[u8], start: usize) -> usize {
    let mut i = start;
    while let Some(&byte) = bytes.get(i) {
        match byte {
            b'\\' => i += 2,
            b'"' => return i + 1,
            _ => i += 1,
        }
    }
    bytes.len()
}

fn to_literal(source: &str, start: usize, end: usize) -> Literal {
    let before = &source[..start];
    let line_start = before.rfind('\n').map_or(0, |newline| newline + 1);
    Literal {
        line: before.matches('\n').count() + 1,
        column: source[line_start..start].chars().count() + 1,
        text: source[start..end].to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn texts(source: &str) -> Vec<String> {
        scan_source(source).into_iter().map(|l| l.text).collect()
    }

    #[test]
    fn flags_colors_lengths_and_durations() {
        let source =
            "Rectangle { background: #ff0000; width: 12px; animate x { duration: 1.5s; } }";
        assert_eq!(texts(source), ["#ff0000", "12px", "1.5s"]);
    }

    #[test]
    fn flags_color_functions_and_named_colors() {
        assert_eq!(
            texts("a: rgb(1, 2, 3); b: Colors.rebeccapurple;"),
            ["rgb", "Colors.rebeccapurple"]
        );
    }

    #[test]
    fn accepts_tokens_zeros_percentages_and_plain_numbers() {
        let source = "x: 0; y: 0px; width: 100%; opacity: 0.5; padding: Theme.spacing.md * 2;";
        assert!(scan_source(source).is_empty());
    }

    #[test]
    fn ignores_comments_strings_and_identifiers() {
        let source = r##"
            // width: 10px;
            /* background: #fff; */
            text: "12px \"#fff\" rgb(0)";
            size-2xl: Theme.typography.h1-size;
        "##;
        assert!(scan_source(source).is_empty());
    }

    #[test]
    fn reports_one_based_positions_in_characters() {
        let literals = scan_source("// é\n  é: 4px;");
        assert_eq!(
            literals,
            [Literal {
                line: 2,
                column: 6,
                text: "4px".into(),
            }]
        );
    }
}
