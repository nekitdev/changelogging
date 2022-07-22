from datetime import date
from pathlib import Path
from typing import Optional

import click
from wraps import Option, convert_optional

from changelogging import __version__ as version
from changelogging.build import Builder
from changelogging.config import Config
from changelogging.defaults import DEFAULT_NAME
from changelogging.git import remove_paths

__all__ = ("changelogging", "build", "create")


def get_config(string: Option[str]) -> Config:
    return Config.from_path(string.map_or_else(Path, Path))


parse_date = date.fromisoformat


def get_date(string: Option[str]) -> date:
    return string.map_or_else(date.today, parse_date)


@click.group(name=DEFAULT_NAME)
@click.help_option("-h", "--help")
@click.version_option(version, "-v", "--version")
def changelogging() -> None:
    pass


@changelogging.command()
@click.help_option("-h", "--help")
@click.option("-c", "--config", "config_path", default=None)
@click.option("-d", "--date", "date_string", default=None)
@click.option("-D", "--draft", is_flag=True, default=False)
@click.option("-r/-n", "--remove/--no-remove", default=False)
def build(
    config_path: Optional[str], date_string: Optional[str], draft: bool, remove: bool
) -> None:
    date = get_date(convert_optional(date_string))

    config = get_config(convert_optional(config_path))

    builder = Builder(config, date)

    if draft:
        click.echo(builder.build())

    else:
        if remove:
            remove_paths(builder.collect_paths())

        builder.write()


HASH = "#"
NEWLINE = "\n"

ABORTED = "Creation aborted."
CREATED = "Created the `{}` fragment."
PLACEHOLDER = "Add the content here."
EDIT = """
# Please enter the fragment content.
# Lines starting with "#" will be ignored.
# Close the file without saving to abort.
"""

NAME = "name"


def is_comment(line: str) -> bool:
    return line.startswith(HASH)


def is_content(line: str) -> bool:
    return not is_comment(line)


concat_newline = NEWLINE.join


@changelogging.command()
@click.help_option("-h", "--help")
@click.option("-c", "--config", "config_path", default=None)
@click.option("-e/-n", "--edit/--no-edit", "edit", default=True)
@click.argument(NAME)
def create(config_path: Optional[str], edit: bool, name: str) -> None:
    config = get_config(convert_optional(config_path))

    if edit:
        string = click.edit(EDIT)

        if string is None:
            click.echo(ABORTED)
            return

        string = concat_newline(map(str.rstrip, filter(is_content, string.splitlines())))

    else:
        string = PLACEHOLDER

    path = config.directory / name

    path.write_text(string.rstrip() + NEWLINE)

    click.echo(CREATED.format(name))
