# Changelog

<!-- changelogging: start -->

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
