<!-- next-header -->

# Changelog

## [Unreleased] - ReleaseDate

### Breaking Changes

Builder-style chaining of attributes has been removed:
```rust
// This does not work anymore
a(href("").class("").id(""))
```

### Internals

Use cargo-release for releasing new versions.

## [0.1.0] - 2024-01-21

This is the initial release of htmf!