from pathlib import Path
from subprocess import call
from typing import Iterable, Sequence

from iters.utils import unary_tuple
from typing_aliases import DynamicTuple

from changelogging.constants import DEFAULT_QUIET

__all__ = ("remove_command", "remove_paths")

GIT = "git"
REMOVE = "rm"
QUIET = "-q"
FORCE = "-f"


def resolve_path(path: Path) -> str:
    return path.resolve().as_posix()


def remove_command(iterable: Iterable[Path], quiet: bool = DEFAULT_QUIET) -> Sequence[str]:
    command: DynamicTuple[str] = (GIT, REMOVE, FORCE)

    if quiet:
        command += unary_tuple(QUIET)

    command += tuple(map(resolve_path, iterable))

    return command


def remove_paths(iterable: Iterable[Path], quiet: bool = DEFAULT_QUIET) -> None:
    call(remove_command(iterable, quiet=quiet))
