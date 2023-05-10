from pathlib import Path
from typing import Optional

import click
from iters.iters import iter
from pendulum import Date
from wraps.option import Option
from wraps.wraps import wrap_optional

from changelogging import __version__ as version
from changelogging.build import Builder
from changelogging.config import Config
from changelogging.constants import DEFAULT_NAME, DEFAULT_QUIET, HASH, NEW_LINE
from changelogging.git import remove_paths
from changelogging.utils import parse_date, right_strip, split_lines, starts_with, today

__all__ = ("changelogging", "build", "create")


def get_config(string: Option[str]) -> Config:
    return Config.from_path(string.map_or_else(Path, Path))


def get_date(string: Option[str]) -> Date:
    return string.map_or_else(today, parse_date)


@click.group(name=DEFAULT_NAME)
@click.help_option("--help", "-h")
@click.version_option(version, "--version", "-V")
def changelogging() -> None:
    pass


CONFIG_PATH = "config_path"
DATE_STRING = "date_string"

REMOVING = "removing `{}`"
removing = REMOVING.format


@changelogging.command()
@click.help_option("--help", "-h")
@click.option("--config", "-c", CONFIG_PATH, default=None)
@click.option("--date", "-d", DATE_STRING, default=None)
@click.option("--quiet", "-q", is_flag=True, default=DEFAULT_QUIET)
@click.option("--draft", "-D", is_flag=True, default=False)
@click.option("--remove/--no-remove", "-r/-n", default=False)
def build(
    config_path: Optional[str], date_string: Optional[str], quiet: bool, draft: bool, remove: bool
) -> None:
    date = get_date(wrap_optional(date_string))

    config = get_config(wrap_optional(config_path))

    builder = Builder(config, date)

    if draft:
        click.echo(builder.build())

    else:
        if remove:
            remove_paths(builder.collect_paths(), quiet=quiet)

        builder.write()


ABORTED = "Creation aborted."
CREATED = "Created the `{}` fragment."
PLACEHOLDER = "Add the content here."
EDIT = """
# Please enter the fragment content.
# Lines starting with `#` will be ignored.
# Close the file without saving to abort.
"""

NAME = "name"


def is_comment(line: str) -> bool:
    return starts_with(line, HASH)


def is_content(line: str) -> bool:
    return not is_comment(line)


@changelogging.command()
@click.help_option("--help", "-h")
@click.option("--config", "-c", CONFIG_PATH, default=None)
@click.option("--edit/--no-edit", "-e/-n", default=True)
@click.argument(NAME)
def create(config_path: Optional[str], edit: bool, name: str) -> None:
    config = get_config(wrap_optional(config_path))

    if edit:
        string = click.edit(EDIT)

        if string is None:
            click.echo(ABORTED)

            return

        string = iter(split_lines(string)).filter(is_content).map(right_strip).join(NEW_LINE)

    else:
        string = PLACEHOLDER

    path = config.directory / name

    path.write_text(right_strip(string) + NEW_LINE)

    click.echo(CREATED.format(name))
