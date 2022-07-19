from os import PathLike
from typing import Callable, Dict, Tuple, TypeVar, Union

from typing_extensions import TypeAlias

__all__ = ("AnyException", "DynamicTuple", "StringDict", "Nullary", "Unary")

T = TypeVar("T")
R = TypeVar("R")

AnyException: TypeAlias = BaseException

DynamicTuple = Tuple[T, ...]

StringDict = Dict[str, T]

Nullary = Callable[[], R]
Unary = Callable[[T], R]

try:
    IntoPath = Union[PathLike[str], str]  # type: ignore

except TypeError:
    IntoPath = Union[PathLike, str]  # type: ignore
