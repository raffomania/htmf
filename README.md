# hypertext markup functions

⚠️ Warning: This is alpha-quality, completely insecure software.

Functions for generating HTML from Rust in a declarative fashion:

```rust
use htmf::prelude::*;

a([href("https://www.rafa.ee"), class("link")])
    .with(text("My Site"))
    .to_html();
```

## Advantages

- Fast compile times
- Leverage standard Rust tooling for formatting, refactoring, linting, syntax highlighting, etc.
- Strong compile-time guarantees
- Easier to debug than custom template languages or macros
- Function calls are visually similar to HTML's structure, making it familiar

## Drawbacks

- It's not clear how to configure tailwind's editor integration to work in .rs files
- Rust's default indentation of 4 spaces is a bit wide for heavily nested html
- Can't copy and paste HTML from other sources
- Long lines, e.g. with many tailwind classes, can cause [rustfmt to give up formatting that line](https://github.com/rust-lang/rustfmt/issues/3863). Using [the nightly format_strings option](https://rust-lang.github.io/rustfmt/?version=v1.6.0&search=#format_strings) can work around some cases. Lowering [the tab_spaces option](https://rust-lang.github.io/rustfmt/?version=v1.6.0&search=#tab_spaces) can help, too

## Getting Started

Coming soon: ways to import htmf, defining attributes, defining contents, converting to html.

After adding htmf to your dependencies, you can get started by declaring a document:

```rust
use htmf::prelude::*;

document().with(html([]).with([
    head([]).with([
        meta([
            name("viewport"), 
            content("width=device-width,initial-scale=1")
        ]),
    ]), 
    body([class("mx-auto mw-xl")])
        .with(main_([]))
]));
```

- htmf provides functions for creating any of the common HTML tags. Tag names conflicting with Rust keywords include a trailing `_` in their name (e.g. `main_`).
- Add children using an element's `with` method. You can pass in arrays of elements, or a single element.
- htmf provides functions to create any of the common HTML attributes. Pass the result to the tag functions directly.

### Importing htmf explicitly

If you prefer to avoid glob imports, you can import htmf modules explicitly:

```rust
use htmf::prelude as h;

h::document();
```

### Defining Elements

The `with` method accepts anything implementing the `IntoElements` trait, including single items implementing `Into<Element>`:

```rust
use htmf::prelude::*;

// Arrays
ul([]).with([li([]), li([])]);

// Single elements
div([]).with(p([]));

// Vectors
ul([]).with(vec![li([]), li([])]);

// Strings
p([]).with("I'm a string!");
p([]).with("I'm a string!".to_string());

// ()
div([]).with(());

// Options
div([]).with(Some(button([])));
div([]).with(None);
```

### Defining Attributes

Tag functions accept anything implementing the `IntoAttrs` trait:

```rust
use htmf::prelude::*;

// Arrays
p([class("prose")]);

// Single attributes
p(class("prose"));

// ()
p(());

// Options
p(Some(class("prose")));
p(None);
```

### Custom Attributes

## Tips for Working with htmf

Coming soon: 2 space indents, structuring code, unstable multiline string formatting.

## Prior Art

- [Elm's html package](https://github.com/elm/html)
- Gleam's [nakai](https://github.com/nakaixo/nakai) and [lustre](https://github.com/lustre-labs/lustre)
- 🦀 [html-rs](https://github.com/ancos2505/html-rs) has a more verbose API than this crate, and uses the builder pattern
- 🦀 [html-builder](https://github.com/asayers/html-builder) uses the `writeln!` macro to build HTML in a string buffer
- 🦀 [build_html](https://github.com/skubalj/build_html) has a slightly more verbose API than this crate, and uses the builder pattern
- [leptos' view builder](https://book.leptos.dev/view/builder.html#no-macros-the-view-builder-syntax) has heavily inspired the API of this crate, but has a much wider scope than just rendering HTML

## Development Setup

For benchmarks, install `cargo-criterion`, e.g. using `cargo install cargo criterion`.

For working on the examples, install a nightly rust toolchain to use the unstable `format_strings` rustfmt option.