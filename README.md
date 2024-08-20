# hypertext markup functions

> [!WARNING]
This is alpha-quality, completely insecure software.

Functions for generating HTML from Rust in a declarative fashion:

```rust
a(
    [href("https://www.rafa.ee"), class("link")],
    [text("My Site")],
)
.to_html();
```

## Advantages

- Fast compile times
- Leverage Rust tooling for formatting, linting, syntax highlighting, ...
- Easier to debug than custom template languages or macros
- Function calls are visually similar to HTML's structure, making it familiar

## Prior Art

- [Elm's html package](https://github.com/elm/html)
- Gleam's [nakai](https://github.com/nakaixo/nakai) and [lustre](https://github.com/lustre-labs/lustre)
- 🦀 [html-rs](https://github.com/ancos2505/html-rs) has a more verbose API than this crate, and uses the builder pattern
- 🦀 [html-builder](https://github.com/asayers/html-builder) uses the `writeln!` macro to build HTML in a string buffer
- 🦀 [build_html](https://github.com/skubalj/build_html) has a slightly more verbose API than this crate, and uses the builder pattern