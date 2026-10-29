# slint-themekit

A small, fully themeable component library for [Slint](https://slint.dev).

Every component reads its colors, fonts, spacing, corner radii, sizes,
opacities and animation timings from one `Theme` global. Assign a different
`ThemeData` value and the whole UI restyles: no component code changes.

The rules every component follows are in
[DESIGN_PRINCIPLES.md](DESIGN_PRINCIPLES.md).

## Components

`Label`, `Button` (primary / secondary / ghost / danger, optional icon),
`Checkbox`, `Radio`, `Switch`, `Slider`, `NumberField` (drag sideways or
type), `TextField` (cuts off a text too long to fit, and tells you so for a
tooltip), `Select` (a dropdown), `Chip`, `Badge`, `Card`, `ProgressBar`,
`Spinner`, `NavItem`, `Alert` (a message in a tone, optionally closable),
`Toast`s in a `ToastStack` (notices floating over the UI, closing by
themselves), `Dialog` (modal, over a scrim), `Popover` (a floating panel for
any content), `Tab` (optionally closable) in a `TabBar` (wraps when the tabs
don't fit), `AccordionItem` (a collapsible section; stack them for an
accordion), `Table` (rows of `TableCell`s under a header, in
`TableColumn`s: texts, numbers to edit, images and buttons; only the rows in
view exist), `ListView` (only the rows in view exist, for
long lists; scrolls sideways for wide rows) of `ListItem`s or of `TreeItem`s
(a tree, flattened), `Menu` (a popup) of `MenuItem`s, `Divider` and
`VerticalDivider`, `Splitter` (a handle that resizes the panes beside it),
`ResizeGrip` (a corner handle that resizes what it sits in), `Shadow` (a
floating panel's shadow, drawn on every renderer), `TooltipText` (the bubble
of Slint's built-in `Tooltip`), `ScrollArea` (scrolls sideways too when its
content is wider), `StateLayer` (hover/press feedback for your own
components) and an `Icons` set drawn from [Lucide](https://lucide.dev).

`Button`, `TextField`, `NumberField` and `Select` come in three sizes
(`size: ControlSize.small`, `medium` or `large`). Controls of one size are
equally tall, so a row of them lines up, and every theme gets the three
sizes from its own tokens.

Interactive controls are *controlled*: a `Checkbox` never flips its own
`checked`, it reports `toggled()`. The state owner decides, which keeps the UI
a pure view of your application state.

## Icons

The icons in `crates/slint-themekit/ui/icons/` are
[Lucide](https://lucide.dev) icons, copied unmodified and distributed under
Lucide's ISC license (`ui/icons/LICENSE`). `just icons` downloads them from the
Lucide release pinned in `just/icons.just`.

## Usage

```toml
# Cargo.toml of your app
[build-dependencies]
slint-build = "1.18"
slint-themekit = "0.1"
```

```rust
// build.rs
use std::collections::HashMap;

fn main() {
    let config = slint_build::CompilerConfiguration::new()
        .with_library_paths(HashMap::from([slint_themekit::library()]));
    slint_build::compile_with_config("ui/app.slint", config).expect("UI compiles");
}
```

```slint
// ui/app.slint
import { Theme, ThemePresets, Button, Card, Label, TextRole } from "@themekit";

export { Theme } // makes the global reachable from Rust

export component App inherits Window {
    background: Theme.palette.background;
    Card {
        Label { text: "Hello"; role: TextRole.title; }
        Button {
            text: "Go dark";
            clicked => { Theme.current = ThemePresets.dark; }
        }
    }
}
```

## Theming

A theme is a `ThemeData` struct (see `crates/slint-themekit/ui/tokens.slint`):
`palette`, `typography`, `spacing`, `radii`, `sizes`, `states`, `elevation`,
`motion`. The palette's `scrim` is laid over the UI behind a `Dialog`. There
are two shadows: `shadow` with `elevation.blur` / `offset` for surfaces lying
on the UI (cards), and the stronger `overlay-shadow` with
`elevation.overlay-blur` / `overlay-offset` for panels floating over it
(dialogs, and your own panels through `Shadow`); zero blur means no
shadow. `motion.toast` is how long a `Toast` stays before it closes by
itself (zero keeps it until the user closes it). Four
presets ship in `ThemePresets`: `light`, `dark`, `sunset` (warm, serif,
roomy, rounded, slow motion) and `terminal` (green on black, monospace,
dense, square, flat, no motion). Sunset and Terminal name DejaVu fonts;
where those aren't installed, they fall back to the system font. To add your
own, declare a `ThemeData` value in a global of your app and assign it to
`Theme.current`, from Slint or from Rust
(`window.global::<Theme>().set_current(..)`).

## Token lint

`slint_themekit::lint` finds hard-coded colors, lengths and durations in
`.slint` files. Use it in a test so components can only use tokens:

```rust
#[test]
fn ui_uses_tokens_only() -> std::io::Result<()> {
    let ui = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("ui");
    let findings = slint_themekit::lint::scan_dir(&ui, &["themes"])?;
    assert!(findings.is_empty(), "{}", slint_themekit::lint::report(&findings));
    Ok(())
}
```

## Gallery

`crates/themekit-gallery` shows every component in every preset. It uses this
library exactly like an app does, and its tests fail if an exported component
is missing from it:

```sh
cargo run -p themekit-gallery
```

## Why a build dependency?

Slint has no built-in way to publish a component library yet (see
[slint-ui/slint#154](https://github.com/slint-ui/slint/issues/154)). This crate
uses what works on stable Slint today:

* the `.slint` files ship inside the crate (`include = ["ui/**/*", ...]`);
* `library()` returns their path from the registry's copy of the crate;
* the app passes it to `with_library_paths`, so `@themekit` resolves at build
  time and the app compiles the sources together with its own UI.

What this approach can't do:

* **Editor support needs a manual setting.** The Slint language server doesn't
  see `build.rs`, so you have to set `slint.libraryPaths` by hand (this
  repository's `.vscode/settings.json` does it).
* **Each app compiles its own copy of the components.** That's fine for a pure
  UI kit. It isn't enough for a library that wants to ship Rust logic wired to
  its own globals, because each app gets its own generated types.

Slint is working on both:

* **Experimental module builds** (`slint-build` feature
  `experimental-module-builds`, tracked in
  [#7060](https://github.com/slint-ui/slint/issues/7060)): the library crate
  compiles its `.slint` files with `.as_library("themekit")` and declares
  `links = "themekit"`. Apps then find `@themekit` through Cargo `DEP_*` metadata
  and reuse the library's generated Rust types. `slint-build` documents it as
  experimental ("may change or be removed"),
  and it still has open bugs ([#14361](https://github.com/slint-ui/slint/issues/14361),
  [#14170](https://github.com/slint-ui/slint/issues/14170)).
* **The `slint-project.json` project file**
  ([#13206](https://github.com/slint-ui/slint/pull/13206)): it declares library
  paths once, for the build, the viewer, the live preview and the language
  server. It was merged after 1.18.1 and isn't released yet.
