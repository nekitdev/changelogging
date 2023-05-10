from pathlib import Path

from pendulum import date

from changelogging.build import Builder
from changelogging.config import Config
from changelogging.constants import DEFAULT_ENCODING, DEFAULT_ERRORS

HERE = Path(__file__).parent

CHANGELOG_NAME = "CHANGELOG.md"
CHANGELOG = HERE / CHANGELOG_NAME

CHANGELOG_RESULT_NAME = "CHANGELOG_RESULT.md"
CHANGELOG_RESULT = HERE / CHANGELOG_RESULT_NAME

TEMPLATE_NAME = "TEMPLATE.md"
TEMPLATE = HERE / TEMPLATE_NAME


BUILDER = Builder(Config.from_path(HERE), date(2005, 1, 13))


def write_template(encoding: str = DEFAULT_ENCODING, errors: str = DEFAULT_ERRORS) -> None:
    CHANGELOG.write_text(TEMPLATE.read_text(encoding, errors), encoding, errors)


def assert_equal(encoding: str = DEFAULT_ENCODING, errors: str = DEFAULT_ERRORS) -> None:
    assert CHANGELOG.read_text(encoding, errors) == CHANGELOG_RESULT.read_text(encoding, errors)


def test_write(encoding: str = DEFAULT_ENCODING, errors: str = DEFAULT_ERRORS) -> None:
    write_template(encoding, errors)  # reset the output

    BUILDER.write(encoding, errors)  # write the output

    assert_equal(encoding, errors)  # test the output

    write_template(encoding, errors)  # reset the output
