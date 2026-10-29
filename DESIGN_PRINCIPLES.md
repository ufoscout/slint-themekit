# Design principles

These rules decide what goes into slint-themekit and how a component is
written. The ones marked **(enforced)** are checked by `cargo test`; the
others are checked in review.

## 1. One theme value drives everything

`Theme.current` holds a `ThemeData` value. Components read every visual value
from the `Theme` global (palette, typography, spacing, radii, sizes, state
opacities, elevation, motion) and from nowhere else.

Switching themes means assigning one value, from Slint or from Rust. A
component never checks which theme is active: `Theme.current.name` and
`Theme.current.dark` are there for apps, not for components.

## 2. Tokens name roles, not values

- Colors are roles: `primary`, `surface`, `text-muted`, `danger`, and so on.
  The names follow Material 3: content drawn on a fill `x` uses `on-x`, and a
  softer variant of `x` is `x-container`.
- Text picks a `TextRole` (caption, body, heading, title, display) and a
  `Tone` (neutral, muted, primary, success, warning, danger). Components get
  the actual size, weight and color from `Theme.font-size()`,
  `Theme.font-weight()` and `Theme.tone-color()`.
- Icons are monochrome and drawn with `colorize:`, so they take the color of
  their context. They all come from one icon set,
  [Lucide](https://lucide.dev), unmodified, so they share one style.
- A new token is added only when no existing role expresses the need.

## 3. A theme is more than a palette

Typography, spacing, radii, sizes, opacities, elevation and motion are tokens
too. So a theme can change density, shape and motion, not only color.
Components must stay correct at any value a theme may pick:

- **Square corners:** `radii.full` may be `0`. Pills use
  `min(Theme.radii.full, self.height / 2)`.
- **Flat surfaces:** elevation may be `0`.
- **No animation:** durations may be `0ms`.
- **Any font:** layouts must not depend on a particular font family.

Adding a field to `ThemeData` affects app themes: Slint fills missing struct
fields with zero values, so a theme written before the field existed gets
`0`. Give new fields a meaning where zero is a safe default, or treat the
change as breaking.

## 4. No hard-coded design values (enforced)

Color literals and non-zero lengths and durations appear only in
`ui/presets.slint`. `slint_themekit::lint` fails `tests/library.rs`
otherwise. Zero, percentages and plain numbers are fine: opacity factors, or
multiples of a token like `Theme.sizes.control-height * 2`. Apps can run the
same lint on their own UI.

## 5. Controls are controlled

A component displays the state it is given and reports what the user wants;
it never changes that state itself:

- `checked`, `selected`, `active` and `value` are `in` properties.
- Intents are callbacks: `Checkbox` emits `toggled()`; `Button`, `Chip` and
  `NavItem` emit `clicked()`.

The owner of the state decides what happens, so the UI stays a pure view of
the application's state. The one exception is editing state that belongs to
the control, like a `TextField`'s draft text.

## 6. One interaction language

- **Hover and press:** `StateLayer` lays the content color over the
  component at `states.hover` or `states.pressed` opacity. No component
  defines its own hover colors.
- **Disabled:** the whole control fades to `states.disabled` opacity.
- **Tints:** tinted backgrounds, like badges, use the tone at `states.tint`
  opacity.
- **Motion:** state changes animate with `motion.fast`, value changes (like
  progress) with `motion.normal`.
- **Sizes:** buttons and input fields take a `size` (`ControlSize`: small,
  medium, large) and read their height, font size and padding from
  `Theme.control-height()`, `Theme.control-font-size()`,
  `Theme.field-padding()` and `Theme.button-padding()`, never from
  `sizes.control-height` directly. Controls of one size are equally tall, so
  a row of them lines up. The three sizes are derived from the theme's
  tokens: density stays the theme's job.

`StateLayer` is public, so apps can build their own interactive surfaces that
behave the same way.

## 7. Accessible by default

Every component exposes its role and state:

- Buttons are `button` and have a default action.
- Checkboxes report `accessible-checked`.
- Selectable items set `accessible-item-selectable` and
  `accessible-item-selected`.
- Progress reports `accessible-value`.

Text that the component already announces gets `accessible-role: none`, so it
is not read twice. Icon-only buttons rely on the app for an
`accessible-label`. Tests drive the UI through the same accessibility API
that screen readers use.

## 8. Independent of the platform style

Nothing is imported from `std-widgets.slint`: std-widgets follow the
platform's style, not the theme. Components are built from Slint's built-in
elements (`Rectangle`, `Text`, `TextInput`, `TouchArea`, `Flickable`,
`Image` and the layouts).

## 9. A small, deliberate public API

`ui/lib.slint` is the only entry point: a file it doesn't re-export is
private. Each public component is a contract with apps, so its properties
and callbacks are named for what they mean to the app (`kind`, `tone`,
`role`, `toggled`), not for how they are drawn.

## 10. Nothing at runtime, nothing to depend on

The crate ships `.slint` sources and has no dependencies. Apps compile the
components in their own build script, registering
`slint_themekit::library()` as a library path. The Rust code only locates
the files and provides the lint.

## 11. The gallery is the first consumer (enforced)

`crates/themekit-gallery` uses the library exactly like an app does (build
script plus `@themekit` imports), so building it checks the library from the
outside. It shows:

- every exported component (`crates/themekit-gallery/tests/gallery.rs`
  fails otherwise);
- every variant and state;
- every preset;

and it is interactive. Slint compiler warnings fail `just check_code`.
