from collections import defaultdict as default_dict
from pathlib import Path
from textwrap import wrap
from typing import Dict, Iterable, Iterator, List, Mapping, Tuple, TypeVar

from attrs import define, field
from funcs.unpacking import unpack_binary
from iters.iters import iter, wrap_iter
from pendulum import Date
from typing_aliases import Binary, Unary

from changelogging.config import Config
from changelogging.constants import (
    DEFAULT_ENCODING,
    DEFAULT_ERRORS,
    DOT,
    DOUBLE_NEW_LINE,
    EMPTY,
    HASH,
    NEW_LINE,
    SPACE,
    WRITE,
)
from changelogging.fragments import Fragment, FragmentType, Issue
from changelogging.utils import left_strip, split_lines, today

__all__ = ("Builder",)

NO_SIGNIFICANT_CHANGES = "No significant changes."

FT = TypeVar("FT", bound=FragmentType)
IT = TypeVar("IT", bound=Issue)

Fragments = Iterable[Fragment[FT, IT]]
Sections = Mapping[FT, Fragments[FT, IT]]


def get_path(item: Tuple[Path, FragmentType]) -> Path:  # pragma: no cover  # `main` only
    path, _ = item

    return path


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
        return iter(fragments).map(self.build_fragment).join(NEW_LINE)

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

    @wrap_iter
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

    def fetch_fragment(
        self,
        path: Path,
        type: FragmentType,
        encoding: str = DEFAULT_ENCODING,
        errors: str = DEFAULT_ERRORS,
    ) -> Fragment[FragmentType, Issue]:
        """Fetches a fragment of `type` from `path`.

        Arguments:
            path: The path to fetch the fragment from.
            type: The type of the fragment to fetch.
            encoding: The encoding to use.
            errors: The error handling strategy to use.

        Returns:
            The fetched fragment.
        """
        issue = self.get_issue(path)
        content = path.read_text(encoding, errors)

        return Fragment(type, content, issue)

    def fetching_fragment(
        self, encoding: str = DEFAULT_ENCODING, errors: str = DEFAULT_ERRORS
    ) -> Binary[Path, FragmentType, Fragment[FragmentType, Issue]]:
        def fetch_fragment(path: Path, type: FragmentType) -> Fragment[FragmentType, Issue]:
            return self.fetch_fragment(path, type, encoding, errors)

        return fetch_fragment

    @wrap_iter
    def collect_fragments(
        self, encoding: str = DEFAULT_ENCODING, errors: str = DEFAULT_ERRORS
    ) -> Fragments[FragmentType, Issue]:
        """Collects fragments from the changes directory specified in the config.

        Arguments:
            encoding: The encoding to use.
            errors: The error handling strategy to use.

        Returns:
            The iterator over the fragments found.
        """
        return (
            self.collect_paths_types()
            .map(unpack_binary(self.fetching_fragment(encoding, errors)))
            .unwrap()
        )

    @wrap_iter
    def collect_paths_types(self) -> Iterator[Tuple[Path, FragmentType]]:
        config = self.config
        directory = config.directory
        types = config.types

        # TODO: perhaps rewrite this using iterators?

        for path in directory.iterdir():
            if path.is_file():
                for suffix in path.suffixes:
                    if types.has_suffix(suffix):
                        yield (path, types.get_suffix(suffix))

    @wrap_iter
    def collect_paths(self) -> Iterator[Path]:
        """Collect paths to fragments.

        Returns:
            The iterator over paths found.
        """
        return self.collect_paths_types().map(get_path).unwrap()  # pragma: no cover  # `main` only

    @classmethod
    def get_issue(cls, path: Path) -> Issue:
        return Issue(int(cls.name_no_suffixes(path)))

    def build(self) -> str:
        """Builds the changelog.

        Returns:
            The build result.
        """
        return self.build_generate(self.collect_sections(self.collect_fragments().unwrap())).join(
            DOUBLE_NEW_LINE
        )

    def write(self, encoding: str = DEFAULT_ENCODING, errors: str = DEFAULT_ERRORS) -> None:
        """Builds the changelog and writes it to the output file.

        Arguments:
            encoding: The encoding to use.
            errors: The error handling strategy to use.
        """
        config = self.config

        output = config.output
        start_string = config.start_string

        content = self.build()

        current = output.read_text(encoding, errors) if output.exists() else EMPTY

        before, start, after = current.partition(start_string)

        with output.open(WRITE, encoding=encoding, errors=errors) as file:
            if not start:  # pragma: no cover  # not tested
                file.write(content + NEW_LINE)

                if current.strip():
                    file.write(NEW_LINE + left_strip(current))

            else:
                file.write(before)
                file.write(start)
                file.write(DOUBLE_NEW_LINE + content + NEW_LINE)

                if after.strip():
                    file.write(NEW_LINE + left_strip(after))

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
    @wrap_iter
    def generate_indent_lines(cls, string: str, bullet: str) -> Iterator[str]:
        if not string:  # pragma: no cover  # not tested
            return

        initial, subsequent = cls.indents(bullet)

        head, *tail = split_lines(string)

        yield (initial + head if head.strip() else head)

        for item in tail:
            yield (subsequent + item if item.strip() else item)

    @classmethod
    def indent_lines(cls, string: str, bullet: str) -> str:
        return cls.generate_indent_lines(string, bullet).join(NEW_LINE)

    @staticmethod
    def wrapping_line(size: int) -> Unary[str, str]:
        def wrap_line(line: str) -> str:
            return iter(wrap(line, size, break_long_words=False)).join(NEW_LINE)

        return wrap_line

    @classmethod
    def indent_wrap_lines(cls, string: str, bullet: str, size: int) -> str:
        size -= iter(cls.indents(bullet)).map(len).max().unwrap()

        return cls.indent_lines(
            iter(split_lines(string)).map(cls.wrapping_line(size)).join(NEW_LINE), bullet
        )
