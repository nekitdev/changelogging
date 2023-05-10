from typing import Sized, Tuple, TypeVar

from pendulum import Date, now, parse

__all__ = (
    "left_strip",
    "right_strip",
    "split_lines",
    "starts_with",
    "unary_tuple",
    "contains_only_item",
    "parse_date",
    "today",
)

left_strip = str.lstrip
"""An alias of [`str.lstrip`][str.lstrip]."""
right_strip = str.rstrip
"""An alias of [`str.rstrip`][str.rstrip]."""

split_lines = str.splitlines
"""An alias of [`str.splitlines`][str.splitlines]."""
starts_with = str.startswith
"""An alias of [`str.startswith`][str.startswith]."""


T = TypeVar("T")


def unary_tuple(item: T) -> Tuple[T]:
    """Creates an unary tuple containing the `item`.

    Returns:
        An unary tuple containing the `item`.
    """
    return (item,)  # pragma: no cover  # used in CLI


ONE = 1


def contains_only_item(items: Sized) -> bool:
    """Checks if there is only one item in `items`.

    Returns:
        Whether there is only one item in `items`."""
    return len(items) == ONE


def parse_date(string: str) -> Date:
    """Parses `YYYY-MM-DD` strings into dates.

    Arguments:
        string: The string to parse.

    Returns:
        The parsed date.
    """
    return parse(string).date()  # type: ignore  # pragma: no cover  # used in CLI


def today() -> Date:
    """Returns the current date.

    Returns:
        The current date.
    """
    return now().date()  # type: ignore  # pragma: no cover  # not tested
