from collections import defaultdict as default_dict
from pathlib import Path
from textwrap import wrap
from typing import Dict, Iterable, Iterator, List, Mapping, Tuple, TypeVar

from attrs import define, field
from pendulum import Date, now
from typing_extensions import Literal

from changelogging.config import Config
from changelogging.constants import DOT, DOUBLE_NEW_LINE, EMPTY, HASH, NEW_LINE, SPACE
from changelogging.fragments import Fragment, FragmentType, Issue

__all__ = ("Builder",)

NO_SIGNIFICANT_CHANGES = "No significant changes."

concat_new_line = NEW_LINE.join
concat_double_new_line = DOUBLE_NEW_LINE.join

WRITE: Literal["w"] = "w"


def today() -> Date:
    return now().date()  # type: ignore


FT = TypeVar("FT", bound=FragmentType)
IT = TypeVar("IT", bound=Issue)

Fragments = Iterable[Fragment[FT, IT]]
Sections = Mapping[FT, Fragments[FT, IT]]


@define()
class Builder:
    """Represents changelog builders."""

    config: Config = field()
    """The config of the builder."""

    date: Date = field(factory=today)
    """The date to use in builds."""

    def build_title(self) -> str:
        """Builds the main title.

        Returns:
            The main title.
        """
        config = self.config

        return self.heading(config.title_level) + config.title_format.format(
            name=config.name, version=config.version, url=config.url, date=self.date
        )

    def build_section_title(self, type: FragmentType) -> str:
        """Builds the section title of the particular `type`.

        Arguments:
            type: The fragment type to build the title for.

        Returns:
            The section title.
        """
        config = self.config

        return self.heading(config.section_level) + type.title

    def build_issue(self, issue: Issue) -> str:
        """Builds the `issue` given.

        Arguments:
            issue: The issue to build.

        Returns:
            The issue built.
        """
        config = self.config

        return config.issue_format.format(issue=issue.value, url=config.url)

    def build_fragment(self, fragment: Fragment[FragmentType, Issue]) -> str:
        """Builds the `fragment` given.

        Arguments:
            fragment: The fragment to build.

        Returns:
            The built fragment.
        """
        config = self.config

        content = config.fragment_format.format(
            content=fragment.content.strip(), issue=self.build_issue(fragment.issue)
        )

        return (
            self.indent_wrap_lines(content, config.bullet, config.wrap_size)
            if config.wrap
            else self.indent_lines(content, config.bullet)
        )

    def build_fragments(self, fragments: Fragments[FragmentType, Issue]) -> str:
        """Builds several `fragments` given.

        Arguments:
            fragments: The fragments to build.

        Returns:
            The fragments built.
        """
        return concat_new_line(map(self.build_fragment, fragments))

    def collect_sections(self, fragments: Fragments[FT, IT]) -> Sections[FT, IT]:
        """Collects `fragments` into sections.

        Arguments:
            fragments: The fragments to collect.

        Returns:
            The collected sections.
        """
        sections: Dict[FT, List[Fragment[FT, IT]]] = default_dict(list)

        for fragment in fragments:
            sections[fragment.type].append(fragment)

        for fragments in sections.values():
            fragments.sort()

        return sections

    def build_generate(self, sections: Sections[FragmentType, Issue]) -> Iterator[str]:
        """Builds `sections`, returning an iterator.

        Arguments:
            sections: The sections to build.

        Returns:
            The iterator over the build result.
        """
        config = self.config

        yield self.build_title()

        empty = True

        for type in config.display.into_types(config.types):
            if type in sections:
                empty = False

                fragments = sections[type]

                yield self.build_section_title(type)
                yield self.build_fragments(fragments)

        if empty:  # pragma: no cover  # not tested
            yield NO_SIGNIFICANT_CHANGES

    def collect_fragments(self) -> Fragments[FragmentType, Issue]:
        """Collects fragments from the changes directory specified in the config.

        Returns:
            The iterator over the fragments found.
        """
        for path, type in self.collect_paths_types():
            issue = self.get_issue(path)
            content = path.read_text()

            yield Fragment(type, content, issue)

    def collect_paths_types(self) -> Iterator[Tuple[Path, FragmentType]]:
        config = self.config
        directory = config.directory
        types = config.types

        for path in directory.iterdir():
            if path.is_file():
                for suffix in path.suffixes:
                    if types.has_suffix(suffix):
                        yield (path, types.get_suffix(suffix))

    def collect_paths(self) -> Iterator[Path]:  # pragma: no cover  # only used in `main`
        """Collect paths to fragments.

        Returns:
            The iterator over paths found.
        """
        for path, _ in self.collect_paths_types():
            yield path

    @classmethod
    def get_issue(cls, path: Path) -> Issue:
        return Issue(int(cls.name_no_suffixes(path)))

    def build(self) -> str:
        """Builds the changelog.

        Returns:
            The build result.
        """
        return concat_double_new_line(
            self.build_generate(self.collect_sections(self.collect_fragments()))
        )

    def write(self) -> None:
        """Builds the changelog and writes it to the output file."""
        config = self.config

        output = config.output
        start_string = config.start_string

        content = self.build()

        current = output.read_text() if output.exists() else EMPTY

        before, start, after = current.partition(start_string)

        with output.open(WRITE) as file:
            if not start:  # pragma: no cover  # not tested
                file.write(content + NEW_LINE)

                if current.strip():
                    file.write(NEW_LINE + current.lstrip())

            else:
                file.write(before)
                file.write(start)
                file.write(DOUBLE_NEW_LINE + content + NEW_LINE)

                if after.strip():
                    file.write(NEW_LINE + after.lstrip())

    @staticmethod
    def heading(level: int) -> str:
        return HASH * level + SPACE

    @staticmethod
    def indents(bullet: str) -> Tuple[str, str]:
        space = SPACE
        return (bullet + space, space + space)

    @staticmethod
    def name_no_suffixes(path: Path) -> str:
        name, _, _ = path.name.partition(DOT)
        return name

    @classmethod
    def generate_indent_lines(cls, string: str, bullet: str) -> Iterator[str]:
        if not string:  # pragma: no cover  # not tested
            return

        initial, subsequent = cls.indents(bullet)

        head, *tail = string.splitlines()

        yield (initial + head if head.strip() else head)

        for item in tail:
            yield (subsequent + item if item.strip() else item)

    @classmethod
    def indent_lines(cls, string: str, bullet: str) -> str:
        return concat_new_line(cls.generate_indent_lines(string, bullet))

    @classmethod
    def indent_wrap_lines(cls, string: str, bullet: str, size: int) -> str:
        size -= max(map(len, cls.indents(bullet)))

        return cls.indent_lines(
            concat_new_line(
                concat_new_line(wrap(line, size, break_long_words=False))
                for line in string.splitlines()
            ),
            bullet,
        )
