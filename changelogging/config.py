from pathlib import Path
from typing import Any, Dict, Iterable, Iterator, Optional, Type, TypeVar, cast, overload

import toml
from attrs import define
from typing_extensions import Never
from versions import Version
from wraps import Option, wrap_optional
from yarl import URL

from changelogging.defaults import (
    DEFAULT_BULLET,
    DEFAULT_DIRECTORY,
    DEFAULT_FRAGMENT_FORMAT,
    DEFAULT_ISSUE_FORMAT,
    DEFAULT_OUTPUT,
    DEFAULT_SECTION_LEVEL,
    DEFAULT_START_STRING,
    DEFAULT_TITLE_FORMAT,
    DEFAULT_TITLE_LEVEL,
    DEFAULT_URL,
    DEFAULT_WRAP,
    DEFAULT_WRAP_SIZE,
)
from changelogging.fragments import DISPLAY, TYPES, AnyFragmentTypes, Display, FragmentType
from changelogging.typing import IntoPath, Unary

__all__ = ("Config", "ConfigData", "AnyConfigData")

CHANGELOGGING = "changelogging.toml"
PYPROJECT = "pyproject.toml"

SEARCH = (CHANGELOGGING, PYPROJECT)

SECTION = "changelogging"

NAME = "name"
VERSION = "version"

TYPE_NAME = "type.name"
TYPE_TITLE = "type.title"

CONFIG_NOT_FOUND = "can not find config in {}"
EXPECTED_FILE_OR_DIRECTORY = "expected either a file or a directory"

SECTION_NOT_FOUND = "can not find `{}` section"
EXPECTED = "expected `{}` to be defined"


def config_not_found(path: Path) -> FileNotFoundError:
    return FileNotFoundError(CONFIG_NOT_FOUND.format(path.resolve().as_posix()))


def section_not_found(section: str) -> ValueError:
    return ValueError(SECTION_NOT_FOUND.format(section))


def expected(name: str) -> ValueError:
    return ValueError(EXPECTED.format(name))


def expected_file_or_directory() -> ValueError:  # pragma: no cover
    return ValueError(EXPECTED_FILE_OR_DIRECTORY)


def empty() -> Iterator[Never]:
    return
    yield  # type: ignore


T = TypeVar("T")


CD = TypeVar("CD", bound="AnyConfigData")


class ConfigData(Dict[str, T]):
    """Dictionaries that support attribute access."""

    def __getattr__(self, name: str) -> Option[T]:
        return wrap_optional(self.get(name))

    def copy(self: CD) -> CD:
        return type(self)(self)


AnyConfigData = ConfigData[Any]


FT = TypeVar("FT", bound=FragmentType)


@overload
def map_to_type(mapping: ConfigData[str]) -> FragmentType:
    ...


@overload
def map_to_type(mapping: ConfigData[str], fragment_type: Type[FT]) -> FT:
    ...


def map_to_type(mapping: ConfigData[str], fragment_type: Type[Any] = FragmentType) -> Any:
    return fragment_type(
        mapping.name.unwrap_or_raise(expected(TYPE_NAME)),
        mapping.title.unwrap_or_raise(expected(TYPE_TITLE)),
    )


C = TypeVar("C", bound="Config")


@define()
class Config:
    name: str
    """The name of the project."""
    version: Version
    """The version of the project."""
    url: URL = URL(DEFAULT_URL)
    """The URL of the project."""
    directory: Path = Path(DEFAULT_DIRECTORY)
    """The `changes` directory."""
    output: Path = Path(DEFAULT_OUTPUT)
    """The output path."""
    title_level: int = DEFAULT_TITLE_LEVEL
    """The title level to use."""
    section_level: int = DEFAULT_SECTION_LEVEL
    """The section title level to use."""
    bullet: str = DEFAULT_BULLET
    """The bullet to use."""
    wrap: bool = DEFAULT_WRAP
    """Whether to wrap lines."""
    wrap_size: int = DEFAULT_WRAP_SIZE
    """The wrap size to use."""
    start_string: str = DEFAULT_START_STRING
    """The start string to look for."""
    title_format: str = DEFAULT_TITLE_FORMAT
    """The format of the title."""
    issue_format: str = DEFAULT_ISSUE_FORMAT
    """The format of the issue."""
    fragment_format: str = DEFAULT_FRAGMENT_FORMAT
    """The format of the fragment."""
    display: Display = DISPLAY
    """The display ordering of fragment types."""
    types: AnyFragmentTypes = TYPES
    """The fragment types to use."""

    # dynamic code ahead...

    @classmethod
    def from_string(cls: Type[C], string: str, source: Optional[Path] = None) -> C:
        """Parses a [`Config`][changelogging.config.Config] from `string`.

        Arguments:
            string: The string to parse.
            source: The source of where the config came from.

        Returns:
            A newly parsed [`Config`][changelogging.config.Config].
        """
        return cls.from_data(cls.parse(string), source)

    @classmethod
    def from_file_path(cls: Type[C], path: IntoPath) -> C:
        """Parses a [`Config`][changelogging.config.Config] from file `path`.

        Arguments:
            path: The path to the config.

        Returns:
            A newly parsed [`Config`][changelogging.config.Config] instance.
        """
        path = Path(path)

        return cls.from_string(path.read_text(), path)

    @classmethod
    def from_path(cls: Type[C], path: IntoPath, search: Iterable[IntoPath] = SEARCH) -> C:
        """Parses a [`Config`][changelogging.config.Config] from `path`.

        If `path` is a directory, this function searches for files in `search` inside of it.

        Arguments:
            path: The path to the config.
            search: The paths to search for.

        Returns:
            A newly parsed [`Config`][changelogging.config.Config].
        """
        path = Path(path)

        if not path.exists():
            raise config_not_found(path)

        if path.is_dir():
            for part in search:
                try_path = path / part

                if try_path.exists():
                    return cls.from_file_path(try_path)

            raise config_not_found(path)

        if path.is_file():
            return cls.from_file_path(path)

        raise expected_file_or_directory()  # pragma: no cover

    @staticmethod
    def parse(string: str) -> AnyConfigData:
        return cast(AnyConfigData, toml.loads(string, AnyConfigData))  # type: ignore

    @classmethod
    def from_data(cls: Type[C], config_dict: AnyConfigData, source: Optional[Path] = None) -> C:
        """Creates a [`Config`][changelogging.config.Config]
        from [`ConfigData`][changelogging.config.ConfigData].

        Arguments:
            config_dict: The config dictionary to use.

        Returns:
            A newly created [`Config`][changelogging.config.Config] instance.
        """
        config_dict = config_dict.tool.unwrap_or(config_dict)
        config = config_dict.changelogging.unwrap_or_raise(section_not_found(SECTION))

        types = AnyFragmentTypes.from_iterable(map(map_to_type, config.types.unwrap_or_else(empty)))

        return cls(
            # `name` and `version` are always required
            name=config.name.unwrap_or_raise(expected(NAME)),
            version=config.version.map(Version.parse).unwrap_or_raise(expected(VERSION)),
            # map to `URL` and `Path` to simplify interaction
            url=config.url.map_or_else(default_url, URL),
            directory=config.directory.map_or_else(default_directory, source_path(source)),
            output=config.output.map_or_else(default_output, source_path(source)),
            # merely return defaults if needed
            title_level=config.title_level.unwrap_or(DEFAULT_TITLE_LEVEL),
            section_level=config.section_level.unwrap_or(DEFAULT_SECTION_LEVEL),
            bullet=config.bullet.unwrap_or(DEFAULT_BULLET),
            wrap=config.wrap.unwrap_or(DEFAULT_WRAP),
            wrap_size=config.wrap_size.unwrap_or(DEFAULT_WRAP_SIZE),
            start_string=config.start_string.unwrap_or(DEFAULT_START_STRING),
            title_format=config.title_format.unwrap_or(DEFAULT_TITLE_FORMAT),
            issue_format=config.issue_format.unwrap_or(DEFAULT_ISSUE_FORMAT),
            fragment_format=config.fragment_format.unwrap_or(DEFAULT_FRAGMENT_FORMAT),
            # map to `Display` type
            display=config.display.map_or(DISPLAY, Display.from_iterable),
            # merge user-defined `types` with already existing default ones
            types=TYPES.merge_with(types),
        )


def default_url() -> URL:
    return URL(DEFAULT_URL)


def default_directory() -> Path:
    return Path(DEFAULT_DIRECTORY)


def default_output() -> Path:
    return Path(DEFAULT_OUTPUT)


def source_path(source: Optional[Path] = None) -> Unary[str, Path]:
    def format_path(string: str) -> Path:
        if source is None:
            return Path(string)

        return Path(string.format(here=source.parent))

    return format_path
