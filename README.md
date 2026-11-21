# `borrowed`

[![license][license-badge]][license]
[![version][version-badge]][changelog]
[![downloads][downloads-badge]][crate]
[![matrix][matrix-badge]][space]
[![docs][docs-badge]][docs]
[![test][test-badge]][actions]

> _Postfix borrowing._

## Installation

You can add `borrowed` as a dependency with the following command:

```console
$ cargo add borrowed
```

Or by directly specifying it in the configuration like so:

```toml
[dependencies]
borrowed = "0.1.0"
```

Alternatively, you can add it directly from the source:

```toml
[dependencies.borrowed]
git = "https://github.com/nekitdev/borrowed.git"
```

## Usage

Firstly, import the trait:

```rust
use borrowed::Borrowed;
```

This trait provides `borrowed` and `borrowed_mut` methods, which take `&self`
and `&mut self` as their receivers respectively, and simply return `self`.

These methods can be used in method chains and other postfix positions.

Note that due to method resolution, calling `borrowed` on `&T` returns `&T` rather
than `&&T`, and the same applies to `borrowed_mut` on `&mut T`.

## Naming

The methods are not named `by_ref` and `by_mut` to avoid shadowing `Iterator::by_ref` and friends.

## Documentation

You can find the documentation [here][docs].

## Support

If you need support with `borrowed`, there are several channels available, in order of preference:

- [`#support:nekit.dev`][support] (matrix channel);
- [`@nekit:nekit.dev`][matrix] (matrix direct);
- [`support@nekit.dev`][email] (support email).

## Changelog

You can find the changelog [here][changelog].

## Security

You can find the Security Policy of `borrowed` [here][policy].

TL;DR: send an email to [`security@nekit.dev`][security].

## Contributing

If you are interested in contributing to `borrowed`, make sure to take a look at the
[Contributing Guide][contributing], as well as the [Code of Conduct][conduct].

## License

`borrowed` is licensed under the MIT License terms. See [License][license] for details.

[email]: mailto:support@nekit.dev
[security]: mailto:security@nekit.dev
[space]: https://nekit.dev/space
[matrix]: https://nekit.dev/matrix
[support]: https://nekit.dev/support
[actions]: https://github.com/nekitdev/borrowed/actions
[changelog]: https://github.com/nekitdev/borrowed/blob/main/CHANGELOG.md
[conduct]: https://github.com/nekitdev/borrowed/blob/main/CODE_OF_CONDUCT.md
[contributing]: https://github.com/nekitdev/borrowed/blob/main/CONTRIBUTING.md
[policy]: https://github.com/nekitdev/borrowed/blob/main/SECURITY.md
[license]: https://github.com/nekitdev/borrowed/blob/main/LICENSE
[crate]: https://crates.io/crates/borrowed
[docs]: https://docs.rs/borrowed
[license-badge]: https://img.shields.io/crates/l/borrowed
[version-badge]: https://img.shields.io/crates/v/borrowed
[downloads-badge]: https://img.shields.io/crates/dr/borrowed
[docs-badge]: https://img.shields.io/docsrs/borrowed
[test-badge]: https://github.com/nekitdev/borrowed/workflows/test/badge.svg
[matrix-badge]: https://img.shields.io/badge/matrix-space-bb77ee
