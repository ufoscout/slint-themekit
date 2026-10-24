//! A small, fully themeable component library for [Slint](https://slint.dev).
//!
//! The crate ships `.slint` files, not Rust widgets. Every component reads its
//! colors, sizes, fonts and timings from the `Theme` global, so an app changes
//! the whole look and feel by assigning one `ThemeData` value.
//!
//! # Using it from an app
//!
//! Add the crate as a *build* dependency and register it as a Slint library:
//!
//! ```rust,ignore
//! // build.rs
//! let config = slint_build::CompilerConfiguration::new()
//!     .with_library_paths(std::collections::HashMap::from([slint_themekit::library()]));
//! slint_build::compile_with_config("ui/app-window.slint", config).expect("UI compiles");
//! ```
//!
//! Then import components in `.slint` files:
//!
//! ```slint,ignore
//! import { Button, Card, Theme } from "@themekit";
//! ```

pub mod lint;

use std::path::PathBuf;

/// Name under which the library is imported: `import { .. } from "@themekit"`.
pub const LIBRARY_NAME: &str = "themekit";

/// Absolute path of the library's entry point (`ui/lib.slint`).
#[must_use]
pub fn library_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("ui")
        .join("lib.slint")
}

/// `(name, path)` pair ready to be collected into the map expected by
/// `slint_build::CompilerConfiguration::with_library_paths`.
#[must_use]
pub fn library() -> (String, PathBuf) {
    (LIBRARY_NAME.to_owned(), library_path())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn library_path_points_to_the_shipped_entry_point() {
        assert!(library_path().is_file());
    }
}
