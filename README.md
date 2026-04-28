# hypertext markup functions

Functions for generating HTML from Rust in a declarative fashion:

```rust
use htmf::prelude::*;

a(
    [href("https://www.rafa.ee"), class("link")],
    text("My Site"),
)
.to_html();
```

Status: beta. The API might change in backwards-incompatible ways, but I mostly like how it works and am using it in [a serious project](github.com/raffomania/ties/).

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

After adding htmf to your dependencies, you can get started by declaring a document:

```rust
use htmf::prelude::*;

document(html(
    [],
    [
        head(
            [],
            [meta([
                name("viewport"),
                content("width=device-width,initial-scale=1"),
            ])],
        ),
        body([class("mx-auto mw-xl")], main_([], [text("Hello htmf!")])),
    ],
));
```

- htmf provides functions for creating any of the common HTML tags. Tag names conflicting with Rust keywords include a trailing `_` in their name (e.g. `main_`).
- Add children using an element's `with` method. You can pass in arrays of elements, or a single element.
- htmf provides functions to create any of the common HTML attributes. Pass the result to the tag functions directly.

### Importing htmf explicitly

If you prefer to avoid glob imports, you can import htmf modules explicitly:

```rust
use htmf::prelude as h;

h::document(());
```

### Defining Elements

Tag functions accept anything that implements `IntoElements` or `Into<Element>`:

```rust
use htmf::prelude::*;

// Single elements
p([], text("Bonjour!"));

// Arrays & Vectors
ul([], [li([], "one"), li([], "two")]);
ul([], vec![li([], "three"), li([], "four")]);

// Strings are automatically converted to `text` elements
p([], text("I'm a text element!"));
p([], "I'm a string slice!");
p([], "I'm a string!".to_string());

// ()
div([], ());

// Options
div([], Some(button([], "press me")));
div([], None);

// nothing() works like `None` or `()`, but is of type `Element`
div([], nothing());
```

### Defining Attributes

Tag functions accept anything implementing the `IntoAttrs` trait:

```rust
use htmf::prelude::*;

// Arrays
p([class("prose")], "text");

// Single attributes
p(class("prose"), "text");

// ()
p((), "empty!");

// Options
p(Some(class("prose")), "some attribute");
p(None, "no attribute");
```

Coming soon: ways to import htmf, defining attributes, defining contents, converting to html.

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