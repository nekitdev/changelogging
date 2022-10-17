from changelogging.constants import EMPTY

__all__ = (
    "DEFAULT_NAME",
    "DEFAULT_URL",
    "DEFAULT_DIRECTORY",
    "DEFAULT_OUTPUT",
    "DEFAULT_TITLE_LEVEL",
    "DEFAULT_SECTION_LEVEL",
    "DEFAULT_BULLET",
    "DEFAULT_WRAP",
    "DEFAULT_WRAP_SIZE",
    "DEFAULT_START_STRING",
    "DEFAULT_TITLE_FORMAT",
    "DEFAULT_ISSUE_FORMAT",
)

DEFAULT_NAME = "changelogging"

DEFAULT_URL = EMPTY
"""The default project URL."""
DEFAULT_DIRECTORY = "changes"
"""The default `changes` directory."""
DEFAULT_OUTPUT = "CHANGELOG.md"
"""The default output path."""
DEFAULT_TITLE_LEVEL = 2
"""The default title level."""
DEFAULT_SECTION_LEVEL = 3
"""The default section title level."""
DEFAULT_BULLET = "-"
"""The default bullet to use."""
DEFAULT_WRAP = False
"""The default `wrap` setting."""
DEFAULT_WRAP_SIZE = 80
"""The default wrap size."""
DEFAULT_START_STRING = "<!-- changelogging: start -->"
"""The default start string."""
DEFAULT_TITLE_FORMAT = "{version} ({date})"
"""The default title format."""
DEFAULT_ISSUE_FORMAT = "#{issue}"
"""The default issue format."""
DEFAULT_FRAGMENT_FORMAT = "{content} ({issue})"
"""The default fragment format."""
