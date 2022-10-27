from pathlib import Path

from changelogging.config import Config, ConfigDict

HERE = Path(__file__).parent

TEST_NAME = "test.toml"

TEST = HERE / TEST_NAME

CONFIG = ConfigDict(  # keep in sync with `tests/test.toml`
    changelogging=ConfigDict(
        name="tests",
        version="0.3.0",
        url="https://github.com/nekitdev/changelogging",
        directory="changes",
        output="CHANGELOG.md",
        start_string="<!-- changelogging: start -->",
        title_format="{version} ({date})",
        issue_format="[#{issue}]({url}/pull/{issue})",
        bullet="-",
        wrap=True,
        wrap_size=100,
        display=["security", "feature", "change", "fix", "deprecation", "removal", "internal"]
    )
)
