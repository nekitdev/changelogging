from pathlib import Path
from typing import Any, Dict, Iterable, Iterator, Type, TypeVar, cast, overload

import toml
from attrs import define
from typing_extensions import Never
from wraps import Option, convert_optional
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
from changelogging.fragment import (
    DISPLAY,
    TYPES,
    AnyFragmentTypes,
    Display,
    FragmentType,
)
from changelogging.typing import IntoPath

__all__ = ("Config", "ConfigDict", "AnyConfigDict")

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


def expected_file_or_directory() -> ValueError:
    return ValueError(EXPECTED_FILE_OR_DIRECTORY)


def empty() -> Iterator[Never]:
    return
    yield  # type: ignore


T = TypeVar("T")


CD = TypeVar("CD", bound="AnyConfigDict")


class ConfigDict(Dict[str, T]):
    def __getattr__(self, name: str) -> Option[T]:
        return convert_optional(self.get(name))

    def copy(self: CD) -> CD:
        return type(self)(self)


AnyConfigDict = ConfigDict[Any]


FT = TypeVar("FT", bound=FragmentType)


@overload
def map_to_type(mapping: ConfigDict[str]) -> FragmentType:
    ...  # pragma: overload


@overload
def map_to_type(mapping: ConfigDict[str], fragment_type: Type[FT]) -> FT:
    ...


def map_to_type(mapping: ConfigDict[str], fragment_type: Type[Any] = FragmentType) -> Any:
    return fragment_type(
        mapping.name.unwrap_or_raise(expected(TYPE_NAME)),
        mapping.title.unwrap_or_raise(expected(TYPE_TITLE)),
    )


C = TypeVar("C", bound="Config")


@define()
class Config:
    name: str
    version: str
    url: URL = URL(DEFAULT_URL)
    directory: Path = Path(DEFAULT_DIRECTORY)
    output: Path = Path(DEFAULT_OUTPUT)
    title_level: int = DEFAULT_TITLE_LEVEL
    section_level: int = DEFAULT_SECTION_LEVEL
    bullet: str = DEFAULT_BULLET
    wrap: bool = DEFAULT_WRAP
    wrap_size: int = DEFAULT_WRAP_SIZE
    start_string: str = DEFAULT_START_STRING
    title_format: str = DEFAULT_TITLE_FORMAT
    issue_format: str = DEFAULT_ISSUE_FORMAT
    fragment_format: str = DEFAULT_FRAGMENT_FORMAT
    display: Display = DISPLAY
    types: AnyFragmentTypes = TYPES

    # dynamic code ahead...

    @classmethod
    def from_string(cls: Type[C], string: str) -> C:
        return cls.from_config_dict(cls.parse_string(string))

    @classmethod
    def from_file_path(cls: Type[C], path: IntoPath) -> C:
        return cls.from_string(Path(path).read_text())

    @classmethod
    def from_path(cls: Type[C], path: IntoPath, search: Iterable[IntoPath] = SEARCH) -> C:
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

        raise expected_file_or_directory()

    @staticmethod
    def parse_string(string: str) -> AnyConfigDict:
        return cast(AnyConfigDict, toml.loads(string, AnyConfigDict))  # type: ignore

    @classmethod
    def from_config_dict(cls: Type[C], config_dict: AnyConfigDict) -> C:
        config_dict = config_dict.tool.unwrap_or(config_dict)
        config = config_dict.changelogging.unwrap_or_raise(section_not_found(SECTION))

        types = AnyFragmentTypes.from_iterable(map(map_to_type, config.types.unwrap_or_else(empty)))

        return cls(
            # `name` and `version` are always required
            name=config.name.unwrap_or_raise(expected(NAME)),
            version=config.version.unwrap_or_raise(expected(VERSION)),
            # map to `URL` and `Path` to simplify interaction
            url=config.url.map_or_else(default_url, URL),
            directory=config.directory.map_or_else(default_directory, Path),
            output=config.output.map_or_else(default_output, Path),
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
