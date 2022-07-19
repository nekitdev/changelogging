from pathlib import Path
from subprocess import call
from typing import Iterable, Sequence

from changelogging.typing import DynamicTuple

__all__ = ("remove_command", "remove_paths")

GIT = "git"
REMOVE = "rm"
QUIET = "--quiet"
FORCE = "--force"


def remove_command(iterable: Iterable[Path]) -> Sequence[str]:
    command: DynamicTuple[str] = (GIT, REMOVE, QUIET, FORCE)

    command += tuple(path.resolve().as_posix() for path in iterable)

    return command


def remove_paths(iterable: Iterable[Path]) -> None:
    call(remove_command(iterable))
