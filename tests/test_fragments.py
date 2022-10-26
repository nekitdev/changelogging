from typing import Iterable, TypeVar

import pytest

from changelogging.fragments import Display, FragmentType, FragmentTypes

SUFFIX = ".{}"
suffix = SUFFIX.format

TEST = "test"
TESTS = "Tests"

TEST_SUFFIX = suffix(TEST)

OTHER = "other"
OTHERS = "Others"

OTHER_SUFFIX = suffix(OTHER)

BROKEN = "broken"
BROKEN_SUFFIX = suffix(BROKEN)


T = TypeVar("T")

FIRST = 0

EMPTY = "iterable is empty"


def first(iterable: Iterable[T]) -> T:
    for item in iterable:
        return item

    raise ValueError(EMPTY)

class TestFragmentType:
    def test_suffix(self) -> None:
        fragment_type = FragmentType(TEST, TESTS)

        assert fragment_type.suffix == suffix(TEST)


class TestFragmentTypes:
    def test_from_types(self) -> None:
        fragment_type = FragmentType(TEST, TESTS)

        fragment_types = FragmentTypes.from_types(fragment_type)

        assert first(fragment_types.types) is fragment_type

    def test_from_iterable(self) -> None:
        fragment_type = FragmentType(OTHER, OTHERS)

        fragment_types = FragmentTypes.from_iterable([fragment_type])

        assert first(fragment_types.types) is fragment_type

    def test_merge_with(self) -> None:
        test_fragment_type = FragmentType(TEST, TESTS)
        other_fragment_type = FragmentType(OTHER, OTHERS)

        test_fragment_types = FragmentTypes.from_types(test_fragment_type)
        other_fragment_types = FragmentTypes.from_types(other_fragment_type)

        fragment_types = FragmentTypes.from_types(test_fragment_type, other_fragment_type)

        assert test_fragment_types.merge_with(other_fragment_types) == fragment_types

    def test_get_name(self) -> None:
        fragment_type = FragmentType(TEST, TESTS)
        fragment_types = FragmentTypes.from_types(fragment_type)

        assert fragment_types.get_name(TEST) is fragment_type

        with pytest.raises(KeyError):
            fragment_types.get_name(BROKEN)

    def test_has_name(self) -> None:
        fragment_types = FragmentTypes.from_types(FragmentType(TEST, TESTS))

        assert fragment_types.has_name(TEST)
        assert not fragment_types.has_name(BROKEN)

    def test_get_suffix(self) -> None:
        fragment_type = FragmentType(TEST, TESTS)
        fragment_types = FragmentTypes.from_types(fragment_type)

        assert fragment_types.get_suffix(TEST_SUFFIX) is fragment_type

        with pytest.raises(KeyError):
            fragment_types.get_suffix(BROKEN_SUFFIX)

    def test_has_suffix(self) -> None:
        fragment_types = FragmentTypes.from_types(FragmentType(TEST, TESTS))

        assert fragment_types.has_suffix(TEST_SUFFIX)
        assert not fragment_types.has_suffix(BROKEN_SUFFIX)


class TestDisplay:
    def test_from_names(self) -> None:
        display = Display.from_names(TEST, OTHER)

        assert display.names == (TEST, OTHER)

    def test_from_iterable(self) -> None:
        names = (TEST, OTHER)

        display = Display.from_iterable(names)

        assert display.names == names

    def test_into_types(self) -> None:
        fragment_type = FragmentType(TEST, TESTS)

        fragment_types = FragmentTypes.from_types(fragment_type)

        display = Display.from_names(TEST)

        assert first(display.into_types(fragment_types)) is fragment_type

        broken = Display.from_names(BROKEN)

        with pytest.raises(LookupError):
            first(broken.into_types(fragment_types))
