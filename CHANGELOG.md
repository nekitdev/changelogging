# Changelog

<!-- changelogging: start -->

## [0.4.4](https://github.com/nekitdev/changelogging/tree/v0.4.4) (2024-08-09)

### Fixes

- `WordSplitter::NoHyphenation` is now used in wrapping options.
  ([#9](https://github.com/nekitdev/changelogging/pull/9))

## [0.4.3](https://github.com/nekitdev/changelogging/tree/v0.4.3) (2024-08-05)

No significant changes.

## [0.4.3](https://github.com/nekitdev/changelogging/tree/v0.4.3) (2024-08-03)

No significant changes.

## [0.4.1](https://github.com/nekitdev/changelogging/tree/v0.4.1) (2024-07-06)

### Fixes

- Fixed `types` computation. ([#7](https://github.com/nekitdev/changelogging/pull/7))

## [0.4.0](https://github.com/nekitdev/changelogging/tree/v0.4.0) (2024-06-27)

### Changes

- Defaults are now specified in code instead of parsing.
  ([#6](https://github.com/nekitdev/changelogging/pull/6))

## [0.3.0](https://github.com/nekitdev/changelogging/tree/v0.3.0) (2024-05-30)

No significant changes.

## [0.2.2](https://github.com/nekitdev/changelogging/tree/v0.2.2) (2024-05-30)

No significant changes.

## [0.2.1](https://github.com/nekitdev/changelogging/tree/v0.2.1) (2024-05-29)

No significant changes.

## [0.2.0](https://github.com/nekitdev/changelogging/tree/v0.2.0) (2024-05-29)

### Features

- Added `git` support.

  In particular, `--add (-a)` option was added to `changelogging create`,
  and `changelogging build` now has two new options: `--stage (-s)` and `--remove (-r)`.
  ([#5](https://github.com/nekitdev/changelogging/pull/5))

### Changes

- Factored out `changelogging build --preview` into `changelogging preview`.
  ([#5](https://github.com/nekitdev/changelogging/pull/5))

### Fixes

- Fixed wrapping and disabled HTML escaping when rendering.
  ([#5](https://github.com/nekitdev/changelogging/pull/5))

## [0.1.0](https://github.com/nekitdev/changelogging/tree/v0.1.0) (2024-05-28)

Initial release.
