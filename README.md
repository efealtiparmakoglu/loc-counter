# loc-counter

**Lines-of-code counter in pure Rust — 33 languages, comment/string aware, zero dependencies.**

[![CI](https://github.com/efealtiparmakoglu/loc-counter/actions/workflows/ci.yml/badge.svg)](https://github.com/efealtiparmakoglu/loc-counter/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)

```
$ loc-counter src
language   files    code  comments  blanks
---------  -------  ----  --------  ------
Rust            4   418        20      46
------------------------------------------------
total           4   418        20      46
```

Counts code, comment and blank lines per language. The language table is
data (extensions + comment syntaxes), and line classification runs a
block-comment state machine with string-literal awareness — a `//` inside a
`"http://..."` string never fools it.

## Why loc-counter

- **33 languages out of the box**, from Rust to SQL to HTML — adding one is
  adding a table row, not code.
- **Comment/string aware.** Block comments (`/* */`, `<!-- -->`, `--[[ ]]`)
  track state across lines; comment openers inside string literals are
  correctly ignored.
- **Formats.** Aligned table, JSON, CSV. Sort by name/files/code/comments/
  blanks.
- **Zero dependencies.** Pure Rust, hand-rolled extension matching.

## Usage

```sh
loc-counter src
loc-counter --ext rs,toml -f json .
loc-counter -s comments .
```

## Development

```sh
cargo test
cargo clippy --all-targets
```

Tests cover the language table sanity, block comments, comment openers
inside strings, hash comments, HTML comment blocks spanning lines, SQL
double-dash, and blank-line accounting inside blocks.

## License

MIT — see [LICENSE](LICENSE).
