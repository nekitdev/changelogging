from pathlib import Path

__all__ = (
    # constants
    "ROOT",
    "EMPTY",
    "SPACE",
    "NEW_LINE",
    "DOUBLE_NEW_LINE",
    "HASH",
    "DOT",
    # defaults
    "DEFAULT_NAME",
    "DEFAULT_ENCODING",
    "DEFAULT_ERRORS",
)

ROOT = Path(__file__).parent

EMPTY = str()
SPACE = " "

NEW_LINE = "\n"

DOUBLE_NEW_LINE = NEW_LINE + NEW_LINE

HASH = "#"

DOT = "."

DEFAULT_IGNORE_REQUIRED = False

DEFAULT_NAME = "changelogging"

DEFAULT_ENCODING = "utf-8"
DEFAULT_ERRORS = "strict"
