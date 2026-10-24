//! The gallery is the library's showcase: every component the library exports
//! must appear in it, and, like any app, it may only use theme tokens.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use slint_themekit::lint;

fn gallery_ui() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("ui")
}

fn is_ident_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '-' || c == '_'
}

fn without_line_comments(source: &str) -> String {
    source
        .lines()
        .map(|line| line.split_once("//").map_or(line, |(code, _)| code))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Components declared with `export component` in the library's files and
/// re-exported by its entry point, `ui/lib.slint`.
fn exported_components() -> io::Result<Vec<String>> {
    let entry_point = slint_themekit::library_path();
    let ui = entry_point.parent().expect("lib.slint lives in ui/");
    let public_api = without_line_comments(&fs::read_to_string(&entry_point)?);

    let mut components = Vec::new();
    for entry in fs::read_dir(ui)? {
        let path = entry?.path();
        if path.extension().is_none_or(|ext| ext != "slint") {
            continue;
        }
        let source = without_line_comments(&fs::read_to_string(&path)?);
        components.extend(
            source
                .lines()
                .filter_map(|line| line.trim().strip_prefix("export component "))
                .filter_map(|rest| rest.split_whitespace().next())
                .filter(|name| mentions(&public_api, name))
                .map(str::to_owned),
        );
    }
    components.sort();
    Ok(components)
}

/// Whether `name` occurs in `source` as a whole identifier.
fn mentions(source: &str, name: &str) -> bool {
    occurrences(source, name).next().is_some()
}

/// Whether `source` instantiates the component `name` (`Name {`).
fn instantiates(source: &str, name: &str) -> bool {
    occurrences(source, name).any(|end| source[end..].trim_start().starts_with('{'))
}

/// End offsets of every whole-identifier occurrence of `name` in `source`.
fn occurrences<'a>(source: &'a str, name: &'a str) -> impl Iterator<Item = usize> + 'a {
    source.match_indices(name).filter_map(move |(start, _)| {
        let end = start + name.len();
        let before = source[..start].chars().next_back();
        let after = source[end..].chars().next();
        let whole = !before.is_some_and(is_ident_char) && !after.is_some_and(is_ident_char);
        whole.then_some(end)
    })
}

#[test]
fn gallery_shows_every_exported_component() -> io::Result<()> {
    let components = exported_components()?;
    assert!(!components.is_empty(), "no exported component found");

    let gallery = without_line_comments(&fs::read_to_string(gallery_ui().join("gallery.slint"))?);
    let missing: Vec<_> = components
        .iter()
        .filter(|name| !instantiates(&gallery, name))
        .collect();
    assert!(
        missing.is_empty(),
        "add these components to examples/gallery/ui/gallery.slint: {missing:?}"
    );
    Ok(())
}

#[test]
fn gallery_reads_every_design_value_from_tokens() -> io::Result<()> {
    let findings = lint::scan_dir(&gallery_ui(), &[])?;
    assert!(findings.is_empty(), "{}", lint::report(&findings));
    Ok(())
}
