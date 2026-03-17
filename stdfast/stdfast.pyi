# flake8: noqa: PYI021
def get_mir(fname):
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
    ```
       import stdfast as sf
       raw_stdf = sf.get_raw_stdf("my_stdf.stdf")
       raw_stdf['master_information']
    ````
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
    ```
       import stdfast as sf
       rows = sf.get_rows("my_stdf.stdf")
       rows[0]
    ````
    """

def parse_stdf(fname):
    """
    parse_stdf(fname: str)
    --

    Parse an STDF file specified by `fname`

    `fname` must be a `str` and may not be a `Path`-like object.

    Returns a dict with keys and values:
       `mir`: `dict` describing the Master Infomation Record (file metadata)
       `df`: `DataFrame` containing the test results
       `test_information`: `DataFrame` containing the merged test information metadata
       `full_test_information`: `dict` containing the full test information metadata

    # Example
    ```
       import stdfast as sf
       stdf = sf.parse_stdf("my_stdf.stdf")
       stdf['df']
    ````
    """
