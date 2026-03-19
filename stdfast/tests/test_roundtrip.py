"""
Roundtrip tests: Python records → write_stdf → binary file → parse_stdf → verify.
"""

import math

import pytest

import stdfast as sf
from stdfast.records import FAR, MIR, MRR, PIR, PRR, PTR, TSR


def _make_records():
    """Return a minimal but representative list of STDF records.

    TSR records must appear before PIR/PTR so that test_num is registered
    with the correct TestType before the second parse pass processes PTRs.
    """
    return [
        FAR(cpu_type=2, stdf_ver=4),
        MIR(
            lot_id="LOT001",
            part_typ="MYPART",
            job_nam="MYJOB",
            tst_temp="25C",
        ),
        
        PIR(head_num=1, site_num=1),
        PTR(
            test_num=1000,
            head_num=1,
            site_num=1,
            result=1.5,
            test_txt="vdd_test",
            lo_limit=1.0,
            hi_limit=2.0,
            units="V",
        ),
        PTR(
            test_num=2000,
            head_num=1,
            site_num=1,
            result=-0.5,
            test_txt="idd_test",
            lo_limit=-1.0,
            hi_limit=0.0,
            units="mA",
        ),
        PRR(
            head_num=1,
            site_num=1,
            hard_bin=1,
            soft_bin=2,
            num_test=2,
            part_id="PART_A",
        ),
        TSR(
            test_num=1000,
            head_num=1,
            site_num=1,
            test_typ="P",
            test_nam="vdd_test",
            exec_cnt=1,
        ),
        TSR(
            test_num=2000,
            head_num=1,
            site_num=1,
            test_typ="P",
            test_nam="idd_test",
            exec_cnt=1,
        ),
        MRR(),
    ]


@pytest.fixture
def roundtrip_stdf(tmp_path):
    """Write records to a tempfile, parse it, and return the result."""
    out_file = str(tmp_path / "roundtrip.stdf")
    records = _make_records()
    sf.write_stdf(out_file, records)
    result = sf.parse_stdf(out_file)
    return result


@pytest.fixture
def roundtrip_stdf_writer(tmp_path):
    """Same as roundtrip_stdf but uses StdfWriter instead of write_stdf."""
    out_file = str(tmp_path / "roundtrip_writer.stdf")
    with sf.StdfWriter(out_file) as w:
        for record in _make_records():
            w.write_record(record)
    result = sf.parse_stdf(out_file)
    return result


# ---------------------------------------------------------------------------
# Master information (MIR fields)
# ---------------------------------------------------------------------------

class TestMasterInformation:
    def test_lot_id(self, roundtrip_stdf):
        assert roundtrip_stdf["master_information"]["lot_id"] == "LOT001"

    def test_part_typ(self, roundtrip_stdf):
        assert roundtrip_stdf["master_information"]["part_typ"] == "MYPART"

    def test_job_nam(self, roundtrip_stdf):
        assert roundtrip_stdf["master_information"]["job_nam"] == "MYJOB"

    def test_tst_temp(self, roundtrip_stdf):
        assert roundtrip_stdf["master_information"]["tst_temp"] == "25C"


# ---------------------------------------------------------------------------
# Data DataFrame (test results)
# ---------------------------------------------------------------------------

class TestDataFrame:
    def test_one_row(self, roundtrip_stdf):
        assert len(roundtrip_stdf["data"]) == 1

    def test_ptr_1000_result(self, roundtrip_stdf):
        value = roundtrip_stdf["data"]["1000"][0]
        assert math.isclose(value, 1.5, rel_tol=1e-5), f"Expected ~1.5, got {value}"

    def test_ptr_2000_result(self, roundtrip_stdf):
        value = roundtrip_stdf["data"]["2000"][0]
        assert math.isclose(value, -0.5, rel_tol=1e-5), f"Expected ~-0.5, got {value}"

    def test_hard_bin(self, roundtrip_stdf):
        assert roundtrip_stdf["data"]["hbin"][0] == 1

    def test_soft_bin(self, roundtrip_stdf):
        assert roundtrip_stdf["data"]["sbin"][0] == 2

    def test_part_id(self, roundtrip_stdf):
        assert roundtrip_stdf["data"]["part_id"][0] == "PART_A"

    def test_head_num(self, roundtrip_stdf):
        assert roundtrip_stdf["data"]["head_num"][0] == 1

    def test_site_num(self, roundtrip_stdf):
        assert roundtrip_stdf["data"]["site_num"][0] == 1


# ---------------------------------------------------------------------------
# Test information DataFrame (metadata)
# ---------------------------------------------------------------------------

class TestInformationFrame:
    def _row_for(self, df, test_num: int) -> dict:
        """Return a dict of column → value for the given test_num."""
        filtered = df.filter(df["test_num"] == test_num)
        assert len(filtered) == 1, f"Expected exactly one row for test_num={test_num}"
        return {col: filtered[col][0] for col in filtered.columns}

    def test_test_1000_present(self, roundtrip_stdf):
        ti = roundtrip_stdf["test_information"]
        assert 1000 in ti["test_num"].to_list()

    def test_test_2000_present(self, roundtrip_stdf):
        ti = roundtrip_stdf["test_information"]
        assert 2000 in ti["test_num"].to_list()

    def test_test_1000_text(self, roundtrip_stdf):
        row = self._row_for(roundtrip_stdf["test_information"], 1000)
        assert row["test_text"] == "vdd_test"

    def test_test_2000_text(self, roundtrip_stdf):
        row = self._row_for(roundtrip_stdf["test_information"], 2000)
        assert row["test_text"] == "idd_test"

    def test_test_1000_low_limit(self, roundtrip_stdf):
        row = self._row_for(roundtrip_stdf["test_information"], 1000)
        assert math.isclose(row["low_limit"], 1.0, rel_tol=1e-5)

    def test_test_1000_high_limit(self, roundtrip_stdf):
        row = self._row_for(roundtrip_stdf["test_information"], 1000)
        assert math.isclose(row["high_limit"], 2.0, rel_tol=1e-5)

    def test_test_1000_units(self, roundtrip_stdf):
        row = self._row_for(roundtrip_stdf["test_information"], 1000)
        assert row["units"] == "V"

    def test_test_2000_units(self, roundtrip_stdf):
        row = self._row_for(roundtrip_stdf["test_information"], 2000)
        assert row["units"] == "mA"


# ---------------------------------------------------------------------------
# StdfWriter roundtrip (mirrors the tests above)
# ---------------------------------------------------------------------------


class TestStdfWriterRoundtrip:
    """Verify StdfWriter produces parse-equivalent output to write_stdf."""

    # Master information
    def test_lot_id(self, roundtrip_stdf_writer):
        assert roundtrip_stdf_writer["master_information"]["lot_id"] == "LOT001"

    def test_part_typ(self, roundtrip_stdf_writer):
        assert roundtrip_stdf_writer["master_information"]["part_typ"] == "MYPART"

    def test_job_nam(self, roundtrip_stdf_writer):
        assert roundtrip_stdf_writer["master_information"]["job_nam"] == "MYJOB"

    def test_tst_temp(self, roundtrip_stdf_writer):
        assert roundtrip_stdf_writer["master_information"]["tst_temp"] == "25C"

    # Data DataFrame
    def test_one_row(self, roundtrip_stdf_writer):
        assert len(roundtrip_stdf_writer["data"]) == 1

    def test_ptr_1000_result(self, roundtrip_stdf_writer):
        value = roundtrip_stdf_writer["data"]["1000"][0]
        assert math.isclose(value, 1.5, rel_tol=1e-5), f"Expected ~1.5, got {value}"

    def test_ptr_2000_result(self, roundtrip_stdf_writer):
        value = roundtrip_stdf_writer["data"]["2000"][0]
        assert math.isclose(value, -0.5, rel_tol=1e-5), f"Expected ~-0.5, got {value}"

    def test_hard_bin(self, roundtrip_stdf_writer):
        assert roundtrip_stdf_writer["data"]["hbin"][0] == 1

    def test_soft_bin(self, roundtrip_stdf_writer):
        assert roundtrip_stdf_writer["data"]["sbin"][0] == 2

    def test_part_id(self, roundtrip_stdf_writer):
        assert roundtrip_stdf_writer["data"]["part_id"][0] == "PART_A"

    # Test information DataFrame
    def test_test_1000_text(self, roundtrip_stdf_writer):
        ti = roundtrip_stdf_writer["test_information"]
        row = ti.filter(ti["test_num"] == 1000)
        assert row["test_text"][0] == "vdd_test"

    def test_test_1000_units(self, roundtrip_stdf_writer):
        ti = roundtrip_stdf_writer["test_information"]
        row = ti.filter(ti["test_num"] == 1000)
        assert row["units"][0] == "V"

    def test_test_2000_units(self, roundtrip_stdf_writer):
        ti = roundtrip_stdf_writer["test_information"]
        row = ti.filter(ti["test_num"] == 2000)
        assert row["units"][0] == "mA"


class TestStdfWriterBytesMatchBatch:
    """StdfWriter must produce bit-for-bit identical output to write_stdf."""

    def test_bytes_equal(self, tmp_path):
        records = _make_records()

        stream_file = str(tmp_path / "stream.stdf")
        with sf.StdfWriter(stream_file) as w:
            for record in records:
                w.write_record(record)

        batch_file = str(tmp_path / "batch.stdf")
        sf.write_stdf(batch_file, records)

        assert open(stream_file, "rb").read() == open(batch_file, "rb").read()

    def test_close_is_idempotent(self, tmp_path):
        out_file = str(tmp_path / "idem.stdf")
        w = sf.StdfWriter(out_file)
        w.write_record(_make_records()[0])
        w.close()
        w.close()  # must not raise

    def test_write_after_close_raises(self, tmp_path):
        out_file = str(tmp_path / "closed.stdf")
        w = sf.StdfWriter(out_file)
        w.close()
        with pytest.raises(OSError):
            w.write_record(_make_records()[0])


# ---------------------------------------------------------------------------
# Shared fixture for iterator tests
# ---------------------------------------------------------------------------


@pytest.fixture
def iter_stdf_file(tmp_path):
    """Write a known record set to disk and return the file path."""
    out_file = str(tmp_path / "iter_test.stdf")
    sf.write_stdf(out_file, _make_records())
    return out_file


# ---------------------------------------------------------------------------
# iter_raw_records tests
# ---------------------------------------------------------------------------


class TestIterRawRecords:
    def test_returns_iterator(self, iter_stdf_file):
        it = sf.iter_raw_records(iter_stdf_file)
        assert hasattr(it, "__iter__")
        assert hasattr(it, "__next__")

    def test_iter_is_self(self, iter_stdf_file):
        it = sf.iter_raw_records(iter_stdf_file)
        assert iter(it) is it

    def test_yields_dicts(self, iter_stdf_file):
        it = sf.iter_raw_records(iter_stdf_file)
        record = next(it)
        assert isinstance(record, dict)

    def test_first_record_has_record_type_key(self, iter_stdf_file):
        record = next(sf.iter_raw_records(iter_stdf_file))
        assert "record_type" in record

    def test_first_record_is_far(self, iter_stdf_file):
        record = next(sf.iter_raw_records(iter_stdf_file))
        assert record["record_type"] == "FAR"

    def test_count_matches_get_raw_records(self, iter_stdf_file):
        eager = sf.get_raw_records(iter_stdf_file)
        lazy = list(sf.iter_raw_records(iter_stdf_file))
        assert len(lazy) == len(eager)

    def test_content_matches_get_raw_records(self, iter_stdf_file):
        # iter_raw_records applies _bytes_to_list; normalise the eager side too
        from stdfast import _bytes_to_list

        eager = [_bytes_to_list(r) for r in sf.get_raw_records(iter_stdf_file)]
        lazy = list(sf.iter_raw_records(iter_stdf_file))
        assert lazy == eager

    def test_ptr_record_has_expected_fields(self, iter_stdf_file):
        ptrs = [
            r for r in sf.iter_raw_records(iter_stdf_file) if r["record_type"] == "PTR"
        ]
        assert len(ptrs) == 2
        ptr = next(p for p in ptrs if p["test_num"] == 1000)
        assert math.isclose(ptr["result"], 1.5, rel_tol=1e-5)
        assert ptr["test_txt"] == "vdd_test"

    def test_stopiteration_when_exhausted(self, iter_stdf_file):
        it = sf.iter_raw_records(iter_stdf_file)
        list(it)  # exhaust
        with pytest.raises(StopIteration):
            next(it)

    def test_file_not_found_raises(self, tmp_path):
        # iter_raw_records is a generator; the file is opened on first next()
        with pytest.raises(OSError):
            next(sf.iter_raw_records(str(tmp_path / "nonexistent.stdf")))


# ---------------------------------------------------------------------------
# iter_records tests (Pydantic-validated models)
# ---------------------------------------------------------------------------


class TestIterRecords:
    def test_returns_iterator(self, iter_stdf_file):
        it = sf.iter_records(iter_stdf_file)
        assert hasattr(it, "__iter__")
        assert hasattr(it, "__next__")

    def test_yields_pydantic_models(self, iter_stdf_file):
        from stdfast.records import FAR as PydanticFAR

        record = next(sf.iter_records(iter_stdf_file))
        assert isinstance(record, PydanticFAR)

    def test_first_record_is_far(self, iter_stdf_file):
        record = next(sf.iter_records(iter_stdf_file))
        assert record.record_type == "FAR"

    def test_count_matches_get_records(self, iter_stdf_file):
        eager = sf.get_records(iter_stdf_file)
        lazy = list(sf.iter_records(iter_stdf_file))
        assert len(lazy) == len(eager)

    def test_content_matches_get_records(self, iter_stdf_file):
        eager = sf.get_records(iter_stdf_file)
        lazy = list(sf.iter_records(iter_stdf_file))
        assert lazy == eager

    def test_ptr_fields_accessible_as_attributes(self, iter_stdf_file):
        ptrs = [r for r in sf.iter_records(iter_stdf_file) if r.record_type == "PTR"]
        assert len(ptrs) == 2
        ptr = next(p for p in ptrs if p.test_num == 1000)  # ty:ignore[unresolved-attribute]
        assert isinstance(ptr, PTR)
        assert math.isclose(ptr.result, 1.5, rel_tol=1e-5)
        assert ptr.test_txt == "vdd_test"
        assert ptr.units == "V"

    def test_stopiteration_when_exhausted(self, iter_stdf_file):
        it = sf.iter_records(iter_stdf_file)
        list(it)  # exhaust
        with pytest.raises(StopIteration):
            next(it)

    def test_file_not_found_raises(self, tmp_path):
        with pytest.raises(OSError):
            next(sf.iter_records(str(tmp_path / "nonexistent.stdf")))
