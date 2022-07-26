from pathlib import Path

import pytest
from versions import parse_version

from changelogging.config import Config, ConfigData

HERE = Path(__file__).parent

BROKEN_NAME = "broken.toml"
BROKEN = HERE / BROKEN_NAME

CHANGELOGGING_NAME = "changelogging.toml"
CHANGELOGGING = HERE / CHANGELOGGING_NAME

DEFAULT_NAME = "default.toml"
DEFAULT = HERE / DEFAULT_NAME

CHANGES_NAME = "changes"
CHANGES = HERE / CHANGES_NAME

NAME = "tests"
VERSION = parse_version("1.0.0")

CONFIG_DATA = ConfigData(  # keep in sync with `tests/changelogging.toml`
    changelogging=ConfigData(
        name="tests",
        version="1.0.0",
        url="https://github.com/nekitdev/changelogging",
        directory="changes",
        output="CHANGELOG.md",
        start_string="<!-- changelogging: start -->",
        title_format="{version} ({date})",
        issue_format="[#{issue}]({url}/pull/{issue})",
        bullet="-",
        wrap=True,
        wrap_size=100,
        display=["security", "feature", "change", "fix", "deprecation", "removal", "internal"],
        types=[ConfigData(name="test", title="Tests")],
    )
)


class TestConfigData:
    def test_copy(self) -> None:
        assert CONFIG_DATA.copy() == CONFIG_DATA


CONFIG = Config.from_data(CONFIG_DATA)


class TestConfig:
    def test_from_path(self) -> None:
        config = Config.from_path(HERE)

        config_direct = Config.from_path(CHANGELOGGING)

        assert config == config_direct

    def test_default(self) -> None:
        config = Config.from_path(DEFAULT)

        assert config.name == NAME
        assert config.version == VERSION

    def test_file_not_found(self) -> None:
        with pytest.raises(FileNotFoundError):
            Config.from_path(BROKEN)

    def test_directory_not_found(self) -> None:
        with pytest.raises(FileNotFoundError):
            Config.from_path(CHANGES)
