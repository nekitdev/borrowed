# Contributing Guide

Thank you for your interest in contributing to `borrowed`!

## Questions

If you have a question, please do not open an issue. Instead, use one of the channels below,
in order of preference:

- [`#support:nekit.dev`][support] (matrix channel);
- [`@nekit:nekit.dev`][matrix] (matrix direct);
- [`support@nekit.dev`][email] (support email).

## Security

**Do not file public issues for security vulnerabilities!**

Please follow the [Security Policy][policy] instead.
TL;DR: send a detailed email to [`security@nekit.dev`][email].

## Issues

Before opening an issue, please search the [existing ones][issues] to avoid duplicates.

When reporting a bug, include:

- the version of `borrowed` and of the Rust toolchain (`rustc --version`);
- a minimal example that reproduces the problem (MRE);
- what you expected to happen, and what happened instead.

## Development

First, install [Rust][rust] via [`rustup`][rustup]. Then clone the repository:

```console
$ git clone https://github.com/nekitdev/borrowed.git
$ cd borrowed
```

Before submitting your changes, make sure that the following commands succeed:

```console
$ cargo fmt --check
$ cargo clippy
$ cargo test
```

Those commands are also run by the CI on Linux, macOS and Windows.

## Changelog

The changelog is built from *fragments* using [`changelogging`][changelogging].

Every pull request with user-facing changes should add a fragment to the `changes` directory,
named `{id}.{type}.md`, where `id` is the pull request number:

```console
$ changelogging create --content "Added cool features!" 13.feature.md
```

The available types are `security`, `feature`, `change`, `fix`, `deprecation`, `removal`
and `internal`.

## Pull Requests

1. Fork the repository and create a branch from `main`.
2. Make your changes, keeping the commits focused.
3. Add tests and documentation where applicable.
4. Add the changelog fragment.
5. Open the pull request, describing what it changes and why.

Please write your pull requests yourself. If you used AI tools while working on them,
say so in the description, along with what they were used for.

## Code of Conduct

By participating in this project, you agree to abide by the [Code of Conduct][conduct].

[support]: https://nekit.dev/support
[matrix]: https://nekit.dev/matrix
[email]: mailto:support@nekit.dev
[policy]: https://github.com/nekitdev/borrowed/blob/main/SECURITY.md
[security]: mailto:security@nekit.dev
[issues]: https://github.com/nekitdev/borrowed/issues
[rust]: https://rust-lang.org/
[rustup]: https://rustup.rs/
[changelogging]: https://github.com/nekitdev/changelogging
[conduct]: https://github.com/nekitdev/borrowed/blob/main/CODE_OF_CONDUCT.md
