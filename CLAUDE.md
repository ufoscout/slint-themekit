# slint-themekit

A themeable Slint component library. The `.slint` files in
`crates/slint-themekit/ui/` are the library; the Rust crate only locates them
(`library()`) and provides a token lint (`lint`). `crates/themekit-gallery`
is a separate crate that consumes the library like an app and shows every
component.

**Read [DESIGN_PRINCIPLES.md](DESIGN_PRINCIPLES.md) before changing anything
in `crates/`.** Every change must respect it. If a request conflicts with a
principle, say which one and propose an alternative instead of breaking it.

## Layout

All code lives in `crates/`, one directory per crate, named after it. The
root `Cargo.toml` only defines the workspace: shared package fields,
dependencies and lints. Inside a crate, paths like `ui/` and `tests/` are
relative to that crate.

```text
crates/slint-themekit/     the library crate
  ui/lib.slint             public entry point: only what it re-exports is public
  ui/tokens.slint          ThemeData and its parts, Tone, TextRole
  ui/theme.slint           the Theme global components read from
  ui/presets.slint         Light, Dark, Sunset, Terminal: the only file with literal values
  ui/<component>.slint     one component per file
  ui/icons.slint           the Icons global: one property per icon
  ui/icons/                Lucide SVGs, unmodified, and Lucide's LICENSE
  src/lint.rs              finds hard-coded colors, lengths and durations
  tests/library.rs         runs the lint on ui/; icons are Lucide's, of one release
  DESIGN_PRINCIPLES.md     symlink to the root copy, so the package ships it
crates/themekit-gallery/   the gallery crate
  ui/gallery.slint         the showcase
  tests/gallery.rs         every exported component is shown; gallery uses tokens only
```

## Commands

```sh
cargo test --workspace                       # lints, gallery coverage, unit tests; builds the gallery
cargo clippy --workspace --all-targets
cargo fmt --all -- --check
cargo run -p themekit-gallery                # look at the result, in every preset
cargo package -p slint-themekit --allow-dirty --list   # before a release: ui/ must be listed
just check_code                              # fmt, clippy and Slint warnings, as CI runs them
just icons                                   # downloads the icons icons.slint references
```

All of them must pass with no warnings. A plain build only prints Slint
warnings; `just slint_check` (part of `just check_code`) builds with
`SLINT_COMPILER_DENY_WARNINGS` and fails on them.

## Adding or changing a component

1. Put a new component in `crates/slint-themekit/ui/<name>.slint`. Import `Theme` from
   `theme.slint` and read every visual value from it.
2. If no token fits, add one to `tokens.slint` and give it a value in every
   preset in `presets.slint`. Zero must be a safe value (principle 3), and
   the new token goes into the README's "Theming" section.
3. Keep it controlled: state comes in through `in` properties, intents go out
   through callbacks. The component never assigns its own state.
4. Hover and press feedback come from `StateLayer`. Disabled controls fade to
   `Theme.states.disabled`. Animations use `Theme.motion`. Set the accessible
   role and state, and give inner text `accessible-role: none`.
5. Export the component, and any enum it needs, from `ui/lib.slint`. List it
   in the README's "Components" section.
6. **Update the gallery in the same change: always, for new components and
   for modified ones.** Edit `crates/themekit-gallery/ui/gallery.slint`:
   - A new component gets a demo, in the card where it fits or in a new one.
   - Every new property, kind, variant, tone or state is shown.
   - If a change alters how a component looks or behaves, adjust its demo so
     the change can be seen and tried.
   - Demo state (checked, selected, values) lives in the gallery and changes
     only in callbacks, like in an app.
   - Run the gallery and check the change in every preset.

   `tests/gallery.rs` only checks that every exported component appears in
   the gallery; showing new variants and states is up to you. If that test
   fails, fix the gallery, never the test.
7. Run the commands above.

## Adding an icon

Icons come from [Lucide](https://lucide.dev), unmodified, at the release
pinned in `just/icons.just` (`LUCIDE_VERSION`).

1. Find the icon on lucide.dev and note its name (e.g. `chevron-down`).
2. Add a property to `Icons` in `ui/icons.slint`, named for what the icon
   means to the library, pointing at the Lucide file:
   `out property <image> expand: @image-url("icons/chevron-down.svg");`
3. Run `just icons`: it downloads every icon `icons.slint` references, and
   Lucide's LICENSE, and removes the ones no longer referenced.
4. Show it in the gallery's "Icons" card.

To move to a newer Lucide release, change `LUCIDE_VERSION`, run `just icons`
and look at the gallery. `tests/library.rs` fails if an icon isn't Lucide's,
if the icons come from different releases, or if `ui/icons/` and
`icons.slint` disagree.

## Changing tokens or presets

- Literal values belong in `presets.slint` only.
- Renaming or removing a token or a `ThemeData` field breaks every app that
  defines a theme. Avoid it; if it is unavoidable, say so explicitly.
- Never widen the lint's exemptions to make a test pass.

## Never

- Import `std-widgets.slint`.
- Hard-code a color, length or duration outside `presets.slint`.
- Let a component change its own `checked`, `selected`, `active` or `value`.
- Make a component check `Theme.current.name` or `Theme.current.dark`.
- Add dependencies or runtime code to the `slint-themekit` crate.
- Add or change a component without updating the gallery.
- Draw or edit an icon by hand: icons come from Lucide through `just icons`.
