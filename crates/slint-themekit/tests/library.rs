//! Guarantees a published version of the library only uses design tokens and
//! Lucide icons.
//!
//! Compiling the library against a real consumer is covered by the gallery
//! (`crates/themekit-gallery`), which builds it from `build.rs` like an app
//! does.

use std::collections::BTreeSet;
use std::fs;
use std::io;
use std::path::PathBuf;

use slint_themekit::lint;

fn ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("ui")
}

#[test]
fn components_only_use_theme_tokens() -> io::Result<()> {
    // `presets.slint` defines the built-in themes: literals belong there.
    let findings = lint::scan_dir(&ui_dir(), &["presets.slint"])?;
    assert!(findings.is_empty(), "{}", lint::report(&findings));
    Ok(())
}

/// The Lucide release an icon comes from, read from the license comment that
/// opens every Lucide SVG. `None` for anything else.
fn lucide_release(svg: &str) -> Option<&str> {
    svg.lines()
        .next()?
        .strip_prefix("<!-- @license lucide-static v")?
        .strip_suffix(" - ISC -->")
}

/// The files `icons.slint` points at (`@image-url("icons/<file>")`).
fn declared_icons(icons_slint: &str) -> BTreeSet<String> {
    icons_slint
        .split("@image-url(\"icons/")
        .skip(1)
        .filter_map(|rest| rest.split_once('"').map(|(file, _)| file.to_owned()))
        .collect()
}

#[test]
fn icons_come_from_one_lucide_release() -> io::Result<()> {
    let icons = ui_dir().join("icons");
    assert!(
        icons.join("LICENSE").is_file(),
        "ui/icons/LICENSE (Lucide's license) is missing: run `just icons`"
    );

    let mut releases = BTreeSet::new();
    let mut files = BTreeSet::new();
    for entry in fs::read_dir(&icons)? {
        let path = entry?.path();
        if path.extension().is_none_or(|ext| ext != "svg") {
            continue;
        }
        let name = path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        let svg = fs::read_to_string(&path)?;
        match lucide_release(&svg) {
            Some(release) => releases.insert(release.to_owned()),
            None => panic!(
                "ui/icons/{name} isn't a Lucide icon: reference it in icons.slint and run `just icons`"
            ),
        };
        files.insert(name);
    }
    assert_eq!(
        releases.len(),
        1,
        "icons from several Lucide releases {releases:?}: run `just icons`"
    );

    let declared = declared_icons(&fs::read_to_string(ui_dir().join("icons.slint"))?);
    assert_eq!(
        files, declared,
        "ui/icons/ and icons.slint list different icons: run `just icons`"
    );
    Ok(())
}
