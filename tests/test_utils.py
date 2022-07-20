from hypothesis import given, strategies

from changelogging.typing import AnyStringDict
from changelogging.utils import mapping_merge


@given(
    strategies.dictionaries(strategies.text(), strategies.from_type(type)),
    strategies.dictionaries(strategies.text(), strategies.from_type(type)),
)
def test_mapping_merge(mapping: AnyStringDict, other: AnyStringDict) -> None:
    result = mapping_merge(mapping, other)

    mapping.update(other)

    assert result == mapping


MAPPING = dict(nekit=13)


def test_mapping_merge_keywords() -> None:
    mapping = MAPPING.copy()

    result = mapping_merge(mapping, nekit=42, other=69)

    mapping.update(nekit=42, other=69)

    assert result == mapping
