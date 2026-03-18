# flake8: noqa: PYI021
def get_mir(fname):
    ...

def get_raw_records(fname: str) -> list:
    """Return the parsed STDF records as a list of dicts, each with a ``record_type`` key."""
    ...

def get_raw_stdf(fname):
    """
    get_raw_stdf(fname: str)
    --

    Parse an STDF file specified by `fname` into a dict structure

    `fname` must be a `str` and may not be a `Path`-like object.

    Returns a nested `dict` representing the raw rust STDF object. Useful if you
    do not need the DataFrame representation and prefer a row-formatted representation.
    The entire `dict` is fully realized, i.e. there are no generators.
       `master_information`: `dict` describing the Master Infomation Record and Master
           Results Record (file metadata)
       `wafer_information`: `dict` describing the Wafer Information Records and Wafer
           Results Records (wafer metadata)
       `site_information`: `dict` describing site information
       `soft_bins`: `dict` of {sbin: SBR}
       `hard_bins`: `dict` of {hbin: HBR}
       `pins`: `dict` of {pin_index: PMR}
       `test_data`: a `dict` describing all of the test results

    # Example
    ```python
       import stdfast as sf
       raw_stdf = sf.get_raw_stdf("my_stdf.stdf")
       raw_stdf['master_information']
    ```
    """

def get_rows(fname):
    """
    get_rows(fname: str)
    --

    Parse an STDF file specified by `fname` and return a list of rows

    `fname` must be a `str` and may not be a `Path`-like object.

    Returns a list of dicts, where each dict represent a single row (i.e. part).
    Useful if you need only the row-formatted data. The list is fully realized,
    i.e. a proper list, not a generator.

    # Example
    ```python
       import stdfast as sf
       rows = sf.get_rows("my_stdf.stdf")
       rows[0]
    ```
    """

def parse_stdf(fname):
    """
    parse_stdf(fname: str)
    --

    Parse an STDF file specified by `fname`

    `fname` must be a `str` and may not be a `Path`-like object.

    Returns a dict with keys and values:
       `master_information`: `dict` describing the Master Information Record and Master
           Results Record (file metadata)
       `site_description`: `dict` describing the Site Description Record, or `None`
       `wafers`: `list` of `dict` describing the Wafer Information Records and Wafer
           Results Records (wafer metadata)
       `soft_bins`: `DataFrame` containing the soft-bin information
       `hard_bins`: `DataFrame` containing the hard-bin information
       `pins`: `DataFrame` containing the pin mapping information
       `pin_mapping`: `dict` mapping test number to a list of pin indices
       `data`: `DataFrame` containing the test results
       `test_information`: `DataFrame` containing the merged test information metadata
       `full_test_information`: `dict` mapping `(test_num, site_num, head_num)` to
           full test information metadata

    # Example
    ```python
       import stdfast as sf
       stdf = sf.parse_stdf("my_stdf.stdf")
       stdf['df']
    ```
    """

def write_stdf(fname, records):
    """
    write_stdf(fname: str, records: list)
    --

    Serialize a list of STDF record objects to a binary STDF file.

    `fname` must be a `str` path to the output file (will be created or overwritten).
    `records` is a list of record model instances from `stdfast.records` (e.g. `FAR`,
    `MIR`, `PTR`, …).  Each object must have a `record_type` attribute matching the
    class name.

    # Example
    ```python
       from stdfast.records import FAR, MIR, MRR, PIR, PTR, PRR
       import stdfast as sf

       records = [
           FAR(cpu_type=2, stdf_ver=4),
           MIR(lot_id="LOT001", part_typ="MY_PART"),
           PIR(head_num=1, site_num=1),
           PTR(test_num=1000, head_num=1, site_num=1, result=3.14, test_txt="vdd"),
           PRR(head_num=1, site_num=1, hard_bin=1, soft_bin=1, num_test=1),
           MRR(),
       ]
       sf.write_stdf("out.stdf", records)
    ```
    """
