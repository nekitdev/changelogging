from typing import Any, Generic, Iterable, Iterator, Type, TypeVar

from attrs import field, frozen

from changelogging.typing import DynamicTuple, StringDict
from changelogging.utils import mapping_merge

__all__ = ("Display", "Fragment", "FragmentType", "FragmentTypes", "AnyFragmentTypes", "Issue")

SUFFIX = ".{}"
suffix = SUFFIX.format


@frozen()
class FragmentType:
    """Represents fragment types."""

    name: str
    """The name of the type."""
    title: str
    """The title of the type."""

    @property
    def suffix(self) -> str:
        """The suffix of the type."""
        return suffix(self.name)


FT = TypeVar("FT", bound=FragmentType)

FragmentTypeTuple = DynamicTuple[FT]
FragmentTypeDict = StringDict[FT]

S = TypeVar("S", bound="AnyFragmentTypes")


@frozen()
class FragmentTypes(Generic[FT]):
    """Represents collections of fragment types."""

    types: FragmentTypeTuple[FT] = ()
    """The contained fragment types."""

    @classmethod
    def from_types(cls: Type[S], *types: FT) -> S:
        """Creates [`FragmentTypes`][changelogging.fragments.FragmentTypes] from `types`.

        Arguments:
            *types: Fragment types to collect.

        Returns:
            A newly created [`FragmentTypes`][changelogging.fragments.FragmentTypes] instance.
        """
        return cls(types)

    @classmethod
    def from_iterable(cls: Type[S], iterable: Iterable[FT]) -> S:
        """Creates [`FragmentTypes`][changelogging.fragments.FragmentTypes]
        from an `iterable` of types.

        Arguments:
            iterable: The iterable to collect types from.

        Returns:
            A newly created [`FragmentTypes`][changelogging.fragments.FragmentTypes] instance.
        """
        return cls(tuple(iterable))

    def merge_with(self: S, other: S) -> S:
        """Merges `self` and `other` into one instance.

        Arguments:
            other: The types to merge with `self`.

        Returns:
            The merged [`FragmentTypes`][changelogging.fragments.FragmentTypes] instance.
        """
        merged = mapping_merge(self.name_to_type, other.name_to_type)

        return self.from_iterable(merged.values())

    @property
    def name_to_type(self) -> FragmentTypeDict[FT]:
        """The `name -> type` mapping."""
        return {type.name: type for type in self.types}

    @property
    def suffix_to_type(self) -> FragmentTypeDict[FT]:
        """The `suffix -> type` mapping."""
        return {type.suffix: type for type in self.types}

    def has_name(self, name: str) -> bool:
        """Checks if the `name` is in [`FragmentTypes`][changelogging.fragments.FragmentTypes].

        Arguments:
            name: The name to check.

        Returns:
            Whether the name is present.
        """
        return name in self.name_to_type

    def get_name(self, name: str) -> FT:
        """Finds a [`FragmentType`][changelogging.fragments.FragmentType] by `name`.

        Arguments:
            name: The name to lookup.

        Raises:
            KeyError: Fragment type was not found.

        Returns:
            The fragment type found.
        """
        return self.name_to_type[name]

    def has_suffix(self, suffix: str) -> bool:
        """Checks if the `suffix` is in [`FragmentTypes`][changelogging.fragments.FragmentTypes].

        Arguments:
            suffix: The suffix to check.

        Returns:
            Whether the suffix is present.
        """
        return suffix in self.suffix_to_type

    def get_suffix(self, suffix: str) -> FT:
        """Finds a [`FragmentType`][changelogging.fragments.FragmentType] by `suffix`.

        Arguments:
            suffix: The suffix to lookup.

        Raises:
            KeyError: Fragment type was not found.

        Returns:
            The fragment type found.
        """
        return self.suffix_to_type[suffix]


AnyFragmentTypes = FragmentTypes[Any]

Names = DynamicTuple[str]

TYPE_NOT_FOUND = "can not find `{}` type"


def type_not_found(name: str) -> LookupError:
    return LookupError(TYPE_NOT_FOUND.format(name))


D = TypeVar("D", bound="Display")


@frozen()
class Display:
    """Represents ordering of fragments' display."""

    names: Names = ()
    """The names of the fragment types."""

    @classmethod
    def from_names(cls: Type[D], *names: str) -> D:
        """Creates a [`Display`][changelogging.fragments.Display] from `names`.

        Arguments:
            *names: The names to use.

        Returns:
            A newly created [`Display`][changelogging.fragments.Display] instance.
        """
        return cls(names)

    @classmethod
    def from_iterable(cls: Type[D], iterable: Iterable[str]) -> D:
        """Creates a [`Display`][changelogging.fragments.Display] from an `iterable` of names.

        Arguments:
            iterable: The iterable to collect names from.

        Returns:
            A newly created [`Display`][changelogging.fragments.Display] instance.
        """
        return cls(tuple(iterable))

    def into_types(self, types: FragmentTypes[FT]) -> Iterator[FT]:
        """Convert names into respective fragment types according to `types`.

        Arguments:
            types: The fragment types to search in.

        Raises:
            LookupError: Fragment type was not found.

        Returns:
            The iterator over fragment types found.
        """
        for name in self.names:
            if types.has_name(name):
                yield types.get_name(name)

            else:
                raise type_not_found(name)


@frozen(order=True)
class Issue:
    """Represents issues."""

    value: int
    """The value of the issue."""


IT = TypeVar("IT", bound=Issue)


@frozen(order=True)
class Fragment(Generic[FT, IT]):
    """Represents fragments."""

    type: FT = field(order=False)
    """The type of the fragment."""
    content: str = field(order=False)
    """The content of the fragment."""
    issue: IT = field(order=True)
    """The issue related to the fragment."""
