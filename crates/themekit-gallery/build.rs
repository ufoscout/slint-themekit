//! Compiles the gallery to Rust exactly like an app consumes the library: at
//! build time, with `@themekit` registered as a Slint library.

use std::collections::HashMap;

fn main() {
    let config = slint_build::CompilerConfiguration::new()
        .with_library_paths(HashMap::from([slint_themekit::library()]));
    slint_build::compile_with_config("ui/gallery.slint", config)
        .expect("the gallery should compile against slint-themekit");
}
