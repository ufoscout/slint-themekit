//! Compiles the gallery to Rust exactly like an app consumes the library: at
//! build time, with `@themekit` registered as a Slint library.

use std::collections::HashMap;

fn main() {
    // slint-build turns warnings into errors when this is set, but doesn't
    // tell Cargo to rerun on a change: without this line, a build that allowed
    // warnings would be reused and `just slint_check` would pass anyway.
    println!("cargo::rerun-if-env-changed=SLINT_COMPILER_DENY_WARNINGS");
    let config = slint_build::CompilerConfiguration::new()
        .with_library_paths(HashMap::from([slint_themekit::library()]));
    slint_build::compile_with_config("ui/gallery.slint", config)
        .expect("the gallery should compile against slint-themekit");
}
