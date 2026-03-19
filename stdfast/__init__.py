from collections.abc import Iterator

from pydantic import TypeAdapter

from .records import Record
from .stdfast import *  # noqa: F403
from .stdfast import get_raw_records as _get_raw_records
from .stdfast import iter_raw_records as _iter_raw_records

_record_adapter: TypeAdapter[Record] = TypeAdapter(Record)


def _bytes_to_list(obj: object) -> object:
    """Recursively convert bytes values to list[int] (Vec<u8> PyO3 compat)."""
    if isinstance(obj, dict):
        return {k: _bytes_to_list(v) for k, v in obj.items()}
    if isinstance(obj, list):
        return [_bytes_to_list(v) for v in obj]
    if isinstance(obj, bytes):
        return list(obj)
    return obj


def get_records(fname: str) -> list[Record]:
    """Parse an STDF file and return a list of Pydantic record models.

    Each element is an instance of the appropriate model from
    ``stdfast.records`` (e.g. ``PTR``, ``MIR``, ``PRR``), determined by
    the ``record_type`` discriminator field.

    :param fname: Path to the STDF file.
    :returns: List of Pydantic record model instances.

    Example::

        import stdfast as sf
        records = sf.get_records("my.stdf")
        ptrs = [r for r in records if r.record_type == "PTR"]
    """
    return [
        _record_adapter.validate_python(_bytes_to_list(r))
        for r in _get_raw_records(fname)
    ]


def iter_raw_records(fname: str) -> Iterator[dict]:
    """Lazily iterate an STDF file, yielding one raw record dict at a time.

    Unlike ``get_raw_records()``, only a single record is held in memory at a
    time. Suitable for very large files (e.g. ~9 million records).

    :param fname: Path to the STDF file.
    :returns: Iterator of dicts, each with a ``record_type`` key.

    Example::

        import stdfast as sf
        for record in sf.iter_raw_records("my.stdf"):
            if record["record_type"] == "PTR":
                process(record)
    """
    for raw in _iter_raw_records(fname):
        yield _bytes_to_list(raw)


def iter_records(fname: str) -> Iterator[Record]:
    """Lazily iterate an STDF file, yielding one validated Pydantic model at a time.

    Memory-efficient alternative to ``get_records()`` for large files.

    :param fname: Path to the STDF file.
    :returns: Iterator of Pydantic record model instances.

    Example::

        import stdfast as sf
        for record in sf.iter_records("my.stdf"):
            if record.record_type == "PTR":
                process(record)
    """
    for raw in _iter_raw_records(fname):
        yield _record_adapter.validate_python(_bytes_to_list(raw))
