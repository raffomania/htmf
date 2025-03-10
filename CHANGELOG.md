<!-- next-header -->

## [Unreleased] - ReleaseDate

### Breaking Changes

- Output unformatted HTML (without newlines or indentation) by default. Add the "pretty-print" feature along with a new method, `to_html_pretty`, for opt-in formatted HTML output.
- Don't render a closing tag for [void elements](https://developer.mozilla.org/en-US/docs/Glossary/Void_element).

# Changelog

## [0.2.0] - 2025-01-21

### Breaking Changes

Builder-style chaining of attributes has been removed:
```rust
// This does not work anymore
a(href("").class("").id(""))
```

### Added

Attributes can now be wrapped in `Option`s:
```rust
// Options
p(Some(class("prose")));
p(None);
```

### Internals

Use cargo-release for releasing new versions.

## [0.1.0] - 2024-01-21

This is the initial release of htmf!