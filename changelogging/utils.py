from typing import Any, Dict, Hashable, Mapping, TypeVar, overload

from pendulum import Date, now

__all__ = ("mapping_merge", "today")

Q = TypeVar("Q", bound=Hashable)
T = TypeVar("T")


@overload
def mapping_merge(*mappings: Mapping[str, T], **keywords: T) -> Dict[str, T]:
    ...


@overload
def mapping_merge(*mappings: Mapping[Q, T]) -> Dict[Q, T]:
    ...


def mapping_merge(*mappings: Mapping[Any, Any], **keywords: Any) -> Dict[Any, Any]:
    """Merges `mappings` and `keywords` into one dictionary.

    Arguments:
        *mappings: Mappings to merge.
        **keywords: Keywords to add to the result.

    Returns:
        A newly created dictionary.
    """
    result: Dict[Any, Any] = {}

    for mapping in mappings:
        result.update(mapping)

    result.update(keywords)

    return result


def today() -> Date:
    """Returns the current date."""
    return now().date()  # type: ignore
