from pathlib import Path
from typing import Any, Dict, Iterable, Optional, Type, TypeVar, cast, overload

import toml
from attrs import define
from iters import iter
from iters.utils import empty
from versions import Version, parse_version
from wraps import Option, wrap_optional
from yarl import URL

from changelogging.constants import (
    DEFAULT_ENCODING,
    DEFAULT_ERRORS,
    DEFAULT_IGNORE_REQUIRED,
    EMPTY,
    ROOT,
)
from changelogging.fragments import AnyFragmentTypes, Display, FragmentType
from changelogging.typing import IntoPath, Unary

__all__ = ("Config", "ConfigData", "AnyConfigData")

CHANGELOGGING = "changelogging.toml"
PYPROJECT = "pyproject.toml"

SEARCH = (CHANGELOGGING, PYPROJECT)

DEFAULT_PATH = ROOT / CHANGELOGGING

CONFIG_NOT_FOUND = "can not find config in {}"
EXPECTED_FILE_OR_DIRECTORY = "expected either a file or a directory"

EXPECTED = "expected `{}`"
expected = EXPECTED.format

EXPECTED_CHANGELOGGING = expected("changelogging")
EXPECTED_CHANGELOGGING_NAME = expected("changelogging.name")
EXPECTED_CHANGELOGGING_VERSION = expected("changelogging.version")
EXPECTED_CHANGELOGGING_URL = expected("changelogging.url")
EXPECTED_CHANGELOGGING_DIRECTORY = expected("changelogging.directory")
EXPECTED_CHANGELOGGING_OUTPUT = expected("changelogging.output")
EXPECTED_CHANGELOGGING_TITLE_LEVEL = expected("changelogging.title_level")
EXPECTED_CHANGELOGGING_SECTION_LEVEL = expected("changelogging.section_level")
EXPECTED_CHANGELOGGING_BULLET = expected("changelogging.bullet")
EXPECTED_CHANGELOGGING_WRAP = expected("changelogging.wrap")
EXPECTED_CHANGELOGGING_WRAP_SIZE = expected("changelogging.wrap_size")
EXPECTED_CHANGELOGGING_START_STRING = expected("changelogging.start_string")
EXPECTED_CHANGELOGGING_TITLE_FORMAT = expected("changelogging.title_format")
EXPECTED_CHANGELOGGING_ISSUE_FORMAT = expected("changelogging.issue_format")
EXPECTED_CHANGELOGGING_FRAGMENT_FORMAT = expected("changelogging.fragment_format")
EXPECTED_CHANGELOGGING_DISPLAY = expected("changelogging.display")
EXPECTED_CHANGELOGGING_TYPES_TYPE_NAME = expected("changelogging.types.type.name")
EXPECTED_CHANGELOGGING_TYPES_TYPE_TITLE = expected("changelogging.types.type.title")


def config_not_found(path: Path) -> FileNotFoundError:
    return FileNotFoundError(CONFIG_NOT_FOUND.format(path.resolve().as_posix()))


def expected_file_or_directory() -> ValueError:  # pragma: no cover
    return ValueError(EXPECTED_FILE_OR_DIRECTORY)


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
def mapping_to_type(mapping: ConfigData[str]) -> FragmentType:
    ...


@overload
def mapping_to_type(mapping: ConfigData[str], fragment_type: Type[FT]) -> FT:
    ...


def mapping_to_type(mapping: ConfigData[str], fragment_type: Type[Any] = FragmentType) -> Any:
    return fragment_type(
        mapping.name.expect(EXPECTED_CHANGELOGGING_TYPES_TYPE_NAME),
        mapping.title.expect(EXPECTED_CHANGELOGGING_TYPES_TYPE_TITLE),
    )


C = TypeVar("C", bound="Config")


@define()
class Config:
    name: str
    """The name of the project."""
    version: Version
    """The version of the project."""
    url: URL
    """The URL of the project."""
    directory: Path
    """The `changes` directory."""
    output: Path
    """The output path."""
    title_level: int
    """The title level to use."""
    section_level: int
    """The section title level to use."""
    bullet: str
    """The bullet to use."""
    wrap: bool
    """Whether to wrap lines."""
    wrap_size: int
    """The wrap size to use."""
    start_string: str
    """The start string to look for."""
    title_format: str
    """The format of the title."""
    issue_format: str
    """The format of the issue."""
    fragment_format: str
    """The format of the fragment."""
    display: Display
    """The display ordering of fragment types."""
    types: AnyFragmentTypes
    """The fragment types to use."""

    # dynamic code ahead...

    @classmethod
    def from_string(cls: Type[C], string: str, source: Optional[Path] = None) -> C:
        """Parses a [`Config`][changelogging.config.Config] from `string`.

        Arguments:
            string: The string to parse.
            source: The source of where the config came from.

        Returns:
            The newly parsed [`Config`][changelogging.config.Config].
        """
        return cls.from_data(cls.parse(string), source)

    @classmethod
    def from_file_path(
        cls: Type[C], path: IntoPath, encoding: str = DEFAULT_ENCODING, errors: str = DEFAULT_ERRORS
    ) -> C:
        """Parses a [`Config`][changelogging.config.Config] from file `path`.

        Arguments:
            path: The path to the config.
            encoding: The encoding to use.
            errors: The error handling strategy to use.

        Returns:
            The newly parsed [`Config`][changelogging.config.Config] instance.
        """
        path = Path(path)

        return cls.from_string(path.read_text(encoding, errors), path)

    @classmethod
    def from_path(
        cls: Type[C],
        path: IntoPath,
        search: Iterable[IntoPath] = SEARCH,
        encoding: str = DEFAULT_ENCODING,
        errors: str = DEFAULT_ERRORS,
    ) -> C:
        """Parses a [`Config`][changelogging.config.Config] from `path`.

        If `path` is a directory, this function searches for files in `search` inside of it.

        Arguments:
            path: The path to the config.
            search: The paths to search for.
            encoding: The encoding to use.
            errors: The error handling strategy to use.

        Returns:
            The newly parsed [`Config`][changelogging.config.Config].
        """
        path = Path(path)

        if not path.exists():
            raise config_not_found(path)

        if path.is_dir():
            for part in search:
                try_path = path / part

                if try_path.exists():
                    return cls.from_file_path(try_path, encoding, errors)

            raise config_not_found(path)

        if path.is_file():
            return cls.from_file_path(path, encoding, errors)

        raise expected_file_or_directory()  # pragma: no cover

    @staticmethod
    def parse(string: str) -> AnyConfigData:
        return cast(AnyConfigData, toml.loads(string, AnyConfigData))

    @classmethod
    def from_data(cls: Type[C], config_data: AnyConfigData, source: Optional[Path] = None) -> C:
        """Creates a [`Config`][changelogging.config.Config]
        from [`ConfigData`][changelogging.config.ConfigData].

        Arguments:
            config_data: The config data to use.

        Returns:
            The newly created [`Config`][changelogging.config.Config] instance.
        """
        config_data = config_data.tool.unwrap_or(config_data)
        config = config_data.changelogging.expect(EXPECTED_CHANGELOGGING)

        types = (
            iter(config.types.unwrap_or_else(empty))
            .map(mapping_to_type)
            .collect(AnyFragmentTypes.from_iterable)
        )

        default_config = DEFAULT_CONFIG

        return cls(
            # `name`, `version` and `url` are always required
            name=config.name.expect(EXPECTED_CHANGELOGGING_NAME),
            version=config.version.map(parse_version).expect(EXPECTED_CHANGELOGGING_VERSION),
            # map to `URL` and `Path` to simplify interaction
            url=config.url.map(URL).expect(EXPECTED_CHANGELOGGING_URL),
            directory=config.directory.map_or(default_config.directory, source_path(source)),
            output=config.output.map_or(default_config.output, source_path(source)),
            # merely return defaults if needed
            title_level=config.title_level.unwrap_or(default_config.title_level),
            section_level=config.section_level.unwrap_or(default_config.section_level),
            bullet=config.bullet.unwrap_or(default_config.bullet),
            wrap=config.wrap.unwrap_or(default_config.wrap),
            wrap_size=config.wrap_size.unwrap_or(default_config.wrap_size),
            start_string=config.start_string.unwrap_or(default_config.start_string),
            title_format=config.title_format.unwrap_or(default_config.title_format),
            issue_format=config.issue_format.unwrap_or(default_config.issue_format),
            fragment_format=config.fragment_format.unwrap_or(default_config.fragment_format),
            # map to `Display` type
            display=config.display.map_or(default_config.display, Display.from_iterable),
            # merge user-defined `types` with already existing default ones
            types=default_config.types.merge_with(types),
        )

    @classmethod
    def unsafe_from_string(
        cls: Type[C],
        string: str,
        source: Optional[Path] = None,
        ignore_required: bool = DEFAULT_IGNORE_REQUIRED,
    ) -> C:
        return cls.unsafe_from_data(cls.parse(string), source, ignore_required=ignore_required)

    @classmethod
    def unsafe_from_file_path(
        cls: Type[C],
        path: IntoPath,
        encoding: str = DEFAULT_ENCODING,
        errors: str = DEFAULT_ERRORS,
        ignore_required: bool = DEFAULT_IGNORE_REQUIRED,
    ) -> C:
        path = Path(path)

        return cls.unsafe_from_string(
            path.read_text(encoding, errors), path, ignore_required=ignore_required
        )

    @classmethod
    def unsafe_from_path(
        cls: Type[C],
        path: IntoPath,
        search: Iterable[IntoPath] = SEARCH,
        encoding: str = DEFAULT_ENCODING,
        errors: str = DEFAULT_ERRORS,
        ignore_required: bool = DEFAULT_IGNORE_REQUIRED,
    ) -> C:
        path = Path(path)

        if not path.exists():  # pragma: no cover
            raise config_not_found(path)

        if path.is_dir():  # pragma: no cover
            for part in search:
                try_path = path / part

                if try_path.exists():
                    return cls.unsafe_from_file_path(
                        try_path, encoding, errors, ignore_required=ignore_required
                    )

            raise config_not_found(path)

        if path.is_file():
            return cls.unsafe_from_file_path(
                path, encoding, errors, ignore_required=ignore_required
            )

        raise expected_file_or_directory()  # pragma: no cover

    @classmethod
    def unsafe_from_data(
        cls: Type[C],
        config_dict: AnyConfigData,
        source: Optional[Path] = None,
        ignore_required: bool = DEFAULT_IGNORE_REQUIRED,
    ) -> C:
        config_dict = config_dict.tool.unwrap_or(config_dict)
        config = config_dict.changelogging.expect(EXPECTED_CHANGELOGGING)

        types = (
            iter(config.types.unwrap_or_else(empty))
            .map(mapping_to_type)
            .collect(AnyFragmentTypes.from_iterable)
        )

        name_option = config.name

        name = (
            name_option.unwrap_or(EMPTY)
            if ignore_required
            else name_option.expect(EXPECTED_CHANGELOGGING_NAME)
        )

        version_option = config.version.map(parse_version)

        version = (
            version_option.unwrap_or_else(Version)
            if ignore_required
            else version_option.expect(EXPECTED_CHANGELOGGING_VERSION)
        )

        url_option = config.url.map(URL)

        url = (
            url_option.unwrap_or_else(URL)
            if ignore_required
            else url_option.expect(EXPECTED_CHANGELOGGING_URL)
        )

        return cls(
            name=name,
            version=version,
            url=url,
            directory=config.directory.map(source_path(source)).expect(
                EXPECTED_CHANGELOGGING_DIRECTORY
            ),
            output=config.output.map(source_path(source)).expect(EXPECTED_CHANGELOGGING_OUTPUT),
            title_level=config.title_level.expect(EXPECTED_CHANGELOGGING_TITLE_LEVEL),
            section_level=config.section_level.expect(EXPECTED_CHANGELOGGING_SECTION_LEVEL),
            bullet=config.bullet.expect(EXPECTED_CHANGELOGGING_BULLET),
            wrap=config.wrap.expect(EXPECTED_CHANGELOGGING_WRAP),
            wrap_size=config.wrap_size.expect(EXPECTED_CHANGELOGGING_WRAP_SIZE),
            start_string=config.start_string.expect(EXPECTED_CHANGELOGGING_START_STRING),
            title_format=config.title_format.expect(EXPECTED_CHANGELOGGING_TITLE_FORMAT),
            issue_format=config.issue_format.expect(EXPECTED_CHANGELOGGING_ISSUE_FORMAT),
            fragment_format=config.fragment_format.expect(EXPECTED_CHANGELOGGING_FRAGMENT_FORMAT),
            display=config.display.map(Display.from_iterable).expect(
                EXPECTED_CHANGELOGGING_DISPLAY
            ),
            types=types,
        )


def source_path(source: Optional[Path] = None) -> Unary[str, Path]:
    def format_path(string: str) -> Path:
        if source is None:
            return Path(string)

        return Path(string.format(here=source.parent))

    return format_path


DEFAULT_CONFIG = Config.unsafe_from_path(DEFAULT_PATH, ignore_required=True)
