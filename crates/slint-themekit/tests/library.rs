//! Guarantees a published version of the library only uses design tokens.
//!
//! Compiling the library against a real consumer is covered by the gallery
//! (`crates/themekit-gallery`), which builds it from `build.rs` like an app
//! does.

use std::path::PathBuf;

use slint_themekit::lint;

#[test]
fn components_only_use_theme_tokens() -> std::io::Result<()> {
    // `presets.slint` defines the built-in themes: literals belong there.
    let ui = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("ui");
    let findings = lint::scan_dir(&ui, &["presets.slint"])?;
    assert!(findings.is_empty(), "{}", lint::report(&findings));
    Ok(())
}
