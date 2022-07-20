from os import PathLike
from typing import Any, Callable, Dict, Tuple, TypeVar, Union

from typing_extensions import TypeAlias

__all__ = ("AnyException", "DynamicTuple", "StringDict", "AnyStringDict", "Nullary", "Unary")

T = TypeVar("T")
R = TypeVar("R")

AnyException: TypeAlias = BaseException

DynamicTuple = Tuple[T, ...]

StringDict = Dict[str, T]

AnyStringDict = StringDict[Any]

Nullary = Callable[[], R]
Unary = Callable[[T], R]

try:  # pragma: no cover
    IntoPath = Union[PathLike[str], str]  # type: ignore

except TypeError:  # pragma: no cover
    IntoPath = Union[PathLike, str]  # type: ignore
