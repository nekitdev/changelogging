from typing import Any, Dict, Hashable, Mapping, TypeVar, overload

__all__ = ("mapping_merge",)

Q = TypeVar("Q", bound=Hashable)
T = TypeVar("T")


@overload
def mapping_merge(*mappings: Mapping[str, T], **keywords: T) -> Dict[str, T]:
    ...


@overload
def mapping_merge(*mappings: Mapping[Q, T]) -> Dict[Q, T]:
    ...


def mapping_merge(*mappings: Mapping[Any, Any], **keywords: Any) -> Dict[Any, Any]:
    result: Dict[Any, Any] = {}

    for mapping in mappings:
        result.update(mapping)

    result.update(keywords)

    return result
