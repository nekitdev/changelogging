from typing import Dict, Generic, Iterable, Type, TypeVar

from attrs import field, frozen

from changelogging.typing import DynamicTuple
from changelogging.utils import mapping_merge

__all__ = (
    "DISPLAY",
    "TYPES",
    "Display",
    "Fragment",
    "FragmentType",
    "FragmentTypes",
    "AnyFragmentTypes",
    "Issue",
)

SUFFIX = ".{}"
format_suffix = SUFFIX.format


@frozen()
class FragmentType:
    name: str
    title: str

    @property
    def suffix(self) -> str:
        return format_suffix(self.name)


SECURITY = FragmentType("security", "Security")
FEATURE = FragmentType("feature", "Features")
CHANGE = FragmentType("change", "Changes")
FIX = FragmentType("fix", "Fixes")
DEPRECATION = FragmentType("deprecation", "Deprecations")
REMOVAL = FragmentType("removal", "Removals")
INTERNAL = FragmentType("internal", "Internal")

DEFAULT = (SECURITY, FEATURE, CHANGE, FIX, DEPRECATION, REMOVAL, INTERNAL)

FT = TypeVar("FT", bound=FragmentType)

FragmentTypeTuple = DynamicTuple[FT]
FragmentTypeDict = Dict[str, FT]

S = TypeVar("S", bound="AnyFragmentTypes")


@frozen()
class FragmentTypes(Generic[FT]):
    types: FragmentTypeTuple[FT] = ()

    @classmethod
    def from_types(cls: Type[S], *types: FT) -> S:
        return cls(types)

    @classmethod
    def from_iterable(cls: Type[S], iterable: Iterable[FT]) -> S:
        return cls(tuple(iterable))

    def merge_with(self: S, other: S) -> S:
        merged = mapping_merge(self.name_to_type, other.name_to_type)

        return self.from_iterable(merged.values())

    @property
    def name_to_type(self) -> FragmentTypeDict[FT]:
        return {type.name: type for type in self.types}

    @property
    def suffix_to_type(self) -> FragmentTypeDict[FT]:
        return {type.suffix: type for type in self.types}

    def has_name(self, name: str) -> bool:
        return name in self.name_to_type

    def get_name(self, name: str) -> FT:
        return self.name_to_type[name]

    def has_suffix(self, suffix: str) -> bool:
        return suffix in self.suffix_to_type

    def get_suffix(self, suffix: str) -> FT:
        return self.suffix_to_type[suffix]


AnyFragmentTypes = FragmentTypes[FragmentType]

Names = DynamicTuple[str]

TYPE_NOT_FOUND = "can not find `{}` type"


def type_not_found(name: str) -> ValueError:
    return ValueError(TYPE_NOT_FOUND.format(name))


D = TypeVar("D", bound="Display")


@frozen()
class Display:
    names: Names = ()

    @classmethod
    def from_names(cls: Type[D], *names: str) -> D:
        return cls(names)

    @classmethod
    def from_iterable(cls: Type[D], iterable: Iterable[str]) -> D:
        return cls(tuple(iterable))

    def into_types(self, types: FragmentTypes[FT]) -> Iterable[FT]:
        for name in self.names:
            if types.has_name(name):
                yield types.get_name(name)

            else:
                raise type_not_found(name)


TYPES = FragmentTypes(DEFAULT)
DISPLAY = Display.from_iterable(type.name for type in DEFAULT)


@frozen(order=True)
class Issue:
    value: int


IT = TypeVar("IT", bound=Issue)


@frozen(order=True)
class Fragment(Generic[FT, IT]):
    type: FT = field(order=False)
    content: str = field(order=False)
    issue: IT = field(order=True)
