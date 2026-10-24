# slint-themekit

A themeable Slint component library. The `.slint` files in `ui/` are the
library; the Rust crate only locates them (`library()`) and provides a token
lint (`lint`). `examples/gallery` is a separate crate that consumes the
library like an app and shows every component.

**Read [DESIGN_PRINCIPLES.md](DESIGN_PRINCIPLES.md) before changing anything
in `ui/` or `examples/gallery/`.** Every change must respect it. If a request
conflicts with a principle, say which one and propose an alternative instead
of breaking it.

## Layout

```text
ui/lib.slint          public entry point: only what it re-exports is public
ui/tokens.slint       ThemeData and its parts, Tone, TextRole
ui/theme.slint        the Theme global components read from
ui/presets.slint      Light and Dark: the only file with literal values
ui/<component>.slint  one component per file
src/lint.rs           finds hard-coded colors, lengths and durations
tests/library.rs      runs the lint on ui/
examples/gallery/     the gallery crate (themekit-gallery)
  ui/gallery.slint    the showcase
  tests/gallery.rs    every exported component is shown; gallery uses tokens only
```

## Commands

```sh
cargo test --workspace                       # lints, gallery coverage, unit tests; builds the gallery
cargo clippy --workspace --all-targets
cargo fmt --all -- --check
cargo run -p themekit-gallery                # look at the result, in Light and Dark
cargo package -p slint-themekit --allow-dirty --list   # before a release: ui/ must be listed
```

All of them must pass with no warnings. Slint warnings already fail the build
(`.cargo/config.toml` sets `SLINT_COMPILER_DENY_WARNINGS`).

## Adding or changing a component

1. Put a new component in `ui/<name>.slint`. Import `Theme` from
   `theme.slint` and read every visual value from it.
2. If no token fits, add one to `tokens.slint` and give it a value in both
   presets in `presets.slint`. Zero must be a safe value (principle 3), and
   the new token goes into the README's "Theming" section.
3. Keep it controlled: state comes in through `in` properties, intents go out
   through callbacks. The component never assigns its own state.
4. Hover and press feedback come from `StateLayer`. Disabled controls fade to
   `Theme.states.disabled`. Animations use `Theme.motion`. Set the accessible
   role and state, and give inner text `accessible-role: none`.
5. Export the component, and any enum it needs, from `ui/lib.slint`. List it
   in the README's "Components" section.
6. **Update the gallery in the same change: always, for new components and
   for modified ones.** Edit `examples/gallery/ui/gallery.slint`:
   - A new component gets a demo, in the card where it fits or in a new one.
   - Every new property, kind, variant, tone or state is shown.
   - If a change alters how a component looks or behaves, adjust its demo so
     the change can be seen and tried.
   - Demo state (checked, selected, values) lives in the gallery and changes
     only in callbacks, like in an app.
   - Run the gallery and check the change in both Light and Dark.

   `tests/gallery.rs` only checks that every exported component appears in
   the gallery; showing new variants and states is up to you. If that test
   fails, fix the gallery, never the test.
7. Run the commands above.

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
