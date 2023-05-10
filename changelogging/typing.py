from os import PathLike
from typing import Dict, TypeVar, Union

__all__ = ("StringDict", "IntoPath")

T = TypeVar("T")

StringDict = Dict[str, T]

try:  # pragma: no cover
    IntoPath = Union[PathLike[str], str]  # type: ignore

except TypeError:  # pragma: no cover
    IntoPath = Union[PathLike, str]  # type: ignore
