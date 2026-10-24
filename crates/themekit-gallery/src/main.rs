//! Interactive showcase of every `slint-themekit` component, in every preset
//! theme.
//!
//! ```text
//! cargo run -p themekit-gallery
//! ```
//!
//! `build.rs` compiles the UI the same way an app would, so building this crate
//! also checks the library from a consumer's point of view.

#[allow(missing_docs, clippy::all, clippy::unwrap_used)]
mod generated {
    slint::include_modules!();
}

use generated::Gallery;
use slint::{ComponentHandle, PlatformError};

fn main() -> Result<(), PlatformError> {
    Gallery::new()?.run()
}
