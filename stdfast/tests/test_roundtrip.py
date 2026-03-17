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
