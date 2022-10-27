from pathlib import Path

from changelogging.build import Builder
from changelogging.config import Config

HERE = Path(__file__).parent

CHANGELOG_NAME = "CHANGELOG.md"
CHANGELOG = HERE / CHANGELOG_NAME

TEMPLATE_NAME = "TEMPLATE.md"
TEMPLATE = HERE / TEMPLATE_NAME

READ = "r"
WRITE = "w"


BUILDER = Builder(Config.from_path(HERE))


def write_template() -> None:
    with TEMPLATE.open(READ) as template, CHANGELOG.open(WRITE) as changelog:
        changelog.write(template.read())


def test_write() -> None:
    write_template()

    BUILDER.write()

    # TODO: perhaps test the output?

    write_template()

