# Usage

## Global Options

- `--help (-h)` displays help information.
- `--version (-V)` shows the application version.

## `create`

To create changelog fragments, one can use the `create` command.

For example, let us create some fragments:

```console
$ changelogging create 13.fix.md
Created the `13.fix.md` fragment.
```

This command will open the editor, where you can edit the fragment content:

```text
Fixed some issues.
# Please enter the fragment content.
# Lines starting with `#` will be ignored.
# Close the file without saving to abort.
```

Now, one can use the `--no-edit (-n)` flag to manually add the fragment content later on.

```console
$ changelogging create 42.feature.md --no-edit
Created the `42.feature.md` fragment.
```

The created fragment is going to contain some placeholder content:

```console
$ cat changes/42.feature.md
Add the content here.
```

Finally, let's add some actual content to the fragment:

```console
$ echo "Added some things." > changes/42.feature.md
```

To sum up, we have created `13.fix.md` and `42.feature.md` changelog fragments.

### Options

- `--config (-c)` specifies the path to the configuration file.
- `--edit (-e)` opens the editor to specify fragment contents (default).
- `--no-edit (-n)` does not open the editor and writes placeholder content instead.

## `build`

This command is used to build the changelog from fragments.

Here is how the changelog file looks like before our build:

```console
$ cat CHANGELOG.md
# Changelog

<!-- changelogging: start -->
```

### Note

Note that it is recommended to have the *start string* (`<!-- changelogging: start -->` by default)
in order for changelogs to generate in a proper order.

And the `changes` directory:

```console
$ ls changes
13.fix.md  42.feature.md
```

To build the changelog in *draft mode* (i.e. without saving changes),
you can use the `--draft (-D)` flag:

```console
$ changelogging build --draft
## 1.0.0 (2022-09-13)

### Features

- Added some things. ([#42](https://github.com/nekitdev/changelogging/pull/42))

### Fixes

- Fixed some issues. ([#13](https://github.com/nekitdev/changelogging/pull/13))
```

Now, let's build the changelog:

```console
$ changelogging build
$ cat CHANGELOG.md
# Changelog

<!-- changelogging: start -->

## 1.0.0 (2022-09-13)

### Features

- Added some things. ([#42](https://github.com/nekitdev/changelogging/pull/42))

### Fixes

- Fixed some issues. ([#13](https://github.com/nekitdev/changelogging/pull/13))
```

### Options

- `--config (-c)` specifies the path to the configuration file.
- `--date (-d)` specifies the date to use when building.
- `--draft (-D)` enables *draft mode* (does not save changes).
- `--remove (-r)` removes the changelog fragments using `git`.
- `--no-remove (-n)` does not remove changelog fragments (default).
