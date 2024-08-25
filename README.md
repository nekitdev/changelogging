![Image]

# `changelogging`

[![License][License Badge]][License]
[![Version][Version Badge]][Crate]
[![PyPI Version][PyPI Version Badge]][PyPI]
[![Downloads][Downloads Badge]][Crate]
[![PyPI Downloads][PyPI Downloads Badge]][PyPI]
[![Discord][Discord Badge]][Discord]
[![Test][Test Badge]][Actions]

> *Building changelogs from fragments.*

## Installation

The binaries can be downloaded from [releases][Releases].

### `pipx`

Note: because `changelogging` was originally written in python, releases on PyPI have
different versions: for instance, the `0.5.0` release is on PyPI with version `2.5.0`,
meaning the major part of the version is always incremented twice to get the PyPI one.

Installing `changelogging` with `pipx` is quite simple:

```console
$ pipx install changelogging
```

Alternatively, the package can be installed from source:

```console
$ pipx install git+https://github.com/nekitdev/changelogging.git
```

Or via cloning the repository:

```console
$ git clone https://github.com/nekitdev/changelogging.git
$ cd changelogging
$ pipx install .
```

### `cargo`

Installing the crate with `cargo` is as simple as with `pipx`:

```console
$ cargo install changelogging
```

Alternatively, the crate can be installed from source:

```console
$ cargo install --git https://github.com/nekitdev/changelogging.git
```

Or via cloning the repository:

```console
$ git clone https://github.com/nekitdev/changelogging.git
$ cd changelogging
$ cargo install --path .
```

## Example

Once `changelogging` is installed, we can start building changelogs!

First things first, we need to configure the *context* of the project.

Create `changelogging.toml` and add the *name*, *version* and *URL* of the project:

```toml
[context]
name = "changelogging"
version = "0.5.0"
url = "https://github.com/nekitdev/changelogging"
```

Then we need to create the `changes` directory.

```console
$ mkdir changes
```

Now we can add *fragments*:

```console
$ changelogging create --content "Added cool features!" 13.feature.md
$ changelogging create --content "Fixed annoying bugs!" 64.fix.md
```

There are also *unlinked* fragments, which have non-integer IDs:

```console
$ changelogging create --content "Fixed security issues!" ~issue.security.md
```

And finally, preview the changelog entry!

```console
$ changelogging preview
## 0.5.0 (YYYY-MM-DD)

### Security

- Fixed security issues!

### Features

- Added cool features! (#13)

### Fixes

- Fixed annoying bugs! (#64)
```

Then let us add our `CHANGELOG.md` file with the following content:

```md
# Changelog

<!-- changelogging: start -->
```

Note that the `start` is essential if we want to add some content before the changelog entries.

Assuming the preview is what we expected it to be, writing it to the changelog is as simple as:

```console
$ changelogging build
```

Finally, let's see the changelog:

```console
$ cat CHANGELOG.md
# Changelog

<!-- changelogging: start -->

## 0.5.0 (YYYY-MM-DD)

### Security

- Fixed security issues!

### Features

- Added cool features! (#13)

### Fixes

- Fixed annoying bugs! (#64)
```

## Documentation

You can find the documentation [here][Documentation].

## Support

If you need support with the library, you can send an [email][Email]
or refer to the official [Discord server][Discord].

## Changelog

You can find the changelog [here][Changelog].

## Security Policy

You can find the Security Policy of `changelogging` [here][Security].

## Contributing

If you are interested in contributing to `changelogging`, make sure to take a look at the
[Contributing Guide][Contributing Guide], as well as the [Code of Conduct][Code of Conduct].

## License

`changelogging` is licensed under the MIT License terms. See [License][License] for details.

[Image]: https://github.com/nekitdev/changelogging/blob/main/changelogging.svg?raw=true

[Email]: mailto:support@nekit.dev

[Discord]: https://nekit.dev/chat

[Actions]: https://github.com/nekitdev/changelogging/actions
[Releases]: https://github.com/nekitdev/changelogging/releases

[Changelog]: https://github.com/nekitdev/changelogging/blob/main/CHANGELOG.md
[Code of Conduct]: https://github.com/nekitdev/changelogging/blob/main/CODE_OF_CONDUCT.md
[Contributing Guide]: https://github.com/nekitdev/changelogging/blob/main/CONTRIBUTING.md
[Security]: https://github.com/nekitdev/changelogging/blob/main/SECURITY.md

[License]: https://github.com/nekitdev/changelogging/blob/main/LICENSE

[Crate]: https://crates.io/crates/changelogging
[PyPI]: https://pypi.org/project/changelogging
[Documentation]: https://docs.rs/changelogging

[Discord Badge]: https://img.shields.io/discord/728012506899021874
[License Badge]: https://img.shields.io/crates/l/changelogging
[Version Badge]: https://img.shields.io/crates/v/changelogging
[PyPI Version Badge]: https://img.shields.io/pypi/v/changelogging
[Downloads Badge]: https://img.shields.io/crates/dr/changelogging
[PyPI Downloads Badge]: https://img.shields.io/pypi/dm/changelogging
[Test Badge]: https://github.com/nekitdev/changelogging/workflows/test/badge.svg
