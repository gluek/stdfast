//! `stdfast_py` contains the Python bindings and code for passing the data to Python
//!
//! The relevant function is `stdfast.parse_stdf`
//!
//! # Example
//! ```python
//! import stdfast as sf
//! stdf = sf.parse_stdf("my_stdf.stdf")
//! stdf['df']
//! ```
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Cursor, Read};

use crate::{
    data::{MasterInformation, Row, STDF, TestData, WaferInformation},
    records::{record_impl::*, Records},
    test_information::TestInformation,
};
use pyo3::prelude::*;
use pyo3_polars::PyDataFrame;

/// A wrapper for the STDF suitable for throwing across the barrier to Python land
#[derive(IntoPyObject)]
struct PySTDF {
    /// MIR and MRR information
    master_information: MasterInformation,
    /// The site information
    site_description: Option<SDR>,
    /// WIR and WRR information
    wafers: Vec<WaferInformation>,
    /// The soft-bin information
    soft_bins: PyDataFrame,
    /// The hard-bin information
    hard_bins: PyDataFrame,
    /// The pin mapping information
    pins: PyDataFrame,
    /// The test number -> [pin_id] mapping
    pin_mapping: HashMap<u32, Vec<u16>>,
    /// The `DataFrame` containing the test results (corresponds to `TestData`)
    data: PyDataFrame,
    /// The `DataFrame` containing the test information metadata (corresponds to
    /// `FullMergedTestInformation`) - TSR and PTR
    test_information: PyDataFrame,
    /// A dict containing the full test information metadata indexed by
    /// (`test_num`, `site_num`, `head_num`)
    full_test_information: HashMap<(u32, u8, u8), TestInformation>,
}

impl PySTDF {
    fn from_stdf(stdf: STDF) -> Self {
        let master_information = stdf.master_information.clone();
        let site_description = stdf.site_information.clone();
        let wafers = stdf.wafer_information.clone();
        let soft_bins = PyDataFrame(stdf.soft_bins_to_df());
        let hard_bins = PyDataFrame(stdf.hard_bins_to_df());
        let pins = PyDataFrame(stdf.pin_mapping_to_df());
        let pin_mapping = stdf.test_data.mpr_index_lookup.clone();
        let test_data = &stdf.test_data;
        let test_info = &test_data.test_information;
        let data = PyDataFrame(test_data.into());
        let test_information = PyDataFrame(test_info.into());
        let full_test_information = stdf.test_data.full_test_information.test_infos;
        Self {
            master_information,
            site_description,
            wafers,
            soft_bins,
            hard_bins,
            pins,
            pin_mapping,
            data,
            test_information,
            full_test_information,
        }
    }

    fn from_fname(fname: &str) -> std::io::Result<Self> {
        Ok(Self::from_stdf(STDF::from_fname(fname)?))
    }

    fn from_bytes(data: Vec<u8>) -> std::io::Result<Self> {
        Ok(Self::from_stdf(STDF::from_reader(Cursor::new(data))?))
    }
}

/// parse_stdf(fname: str)
/// --
///
/// Parse an STDF file specified by `fname`
///
/// `fname` must be a `str` and may not be a `Path`-like object.
///
/// Returns a dict with keys and values:
///    `master_information`: `dict` describing the Master Information Record and Master
///        Results Record (file metadata)
///    `site_description`: `dict` describing the Site Description Record, or `None`
///    `wafers`: `list` of `dict` describing the Wafer Information Records and Wafer
///        Results Records (wafer metadata)
///    `soft_bins`: `DataFrame` containing the soft-bin information
///    `hard_bins`: `DataFrame` containing the hard-bin information
///    `pins`: `DataFrame` containing the pin mapping information
///    `pin_mapping`: `dict` mapping test number to a list of pin indices
///    `data`: `DataFrame` containing the test results
///    `test_information`: `DataFrame` containing the merged test information metadata
///    `full_test_information`: `dict` mapping `(test_num, site_num, head_num)` to
///        full test information metadata
///
/// # Example
/// ```python
///    import stdfast as sf
///    stdf = sf.parse_stdf("my_stdf.stdf")
///    stdf['df']
/// ```
#[pyfunction]
fn parse_stdf(fname: &Bound<'_, PyAny>) -> PyResult<PySTDF> {
    let pystdf = match resolve_source(fname)? {
        Source::Path(path) => PySTDF::from_fname(&path)?,
        Source::Bytes(bytes) => PySTDF::from_bytes(bytes)?,
    };
    Ok(pystdf)
}

/// get_rows(fname: str)
/// --
///
/// Parse an STDF file specified by `fname` and return a list of rows
///
/// `fname` must be a `str` and may not be a `Path`-like object.
///
/// Returns a list of dicts, where each dict represent a single row (i.e. part).
/// Useful if you need only the row-formatted data. The list is fully realized,
/// i.e. a proper list, not a generator.
///
/// # Example
/// ```python
///    import stdfast as sf
///    rows = sf.get_rows("my_stdf.stdf")
///    rows[0]
/// ```
#[pyfunction]
fn get_rows(fname: &Bound<'_, PyAny>) -> PyResult<Vec<Row>> {
    let test_data = match resolve_source(fname)? {
        Source::Path(path) => TestData::from_fname(&path, false)?,
        Source::Bytes(bytes) => TestData::from_bytes(&bytes, false)?,
    };
    Ok(test_data.data)
}

/// get_raw_stdf(fname: str)
/// --
///
/// Parse an STDF file specified by `fname` into a dict structure
///
/// `fname` must be a `str` and may not be a `Path`-like object.
///
/// Returns a nested `dict` representing the raw rust STDF object. Useful if you
/// do not need the DataFrame representation and prefer a row-formatted representation.
/// The entire `dict` is fully realized, i.e. there are no generators.
///    `master_information`: `dict` describing the Master Infomation Record and Master
///        Results Record (file metadata)
///    `wafer_information`: `dict` describing the Wafer Information Records and Wafer
///        Results Records (wafer metadata)
///    `site_information`: `dict` describing site information
///    `soft_bins`: `dict` of {sbin: SBR}
///    `hard_bins`: `dict` of {hbin: HBR}
///    `pins`: `dict` of {pin_index: PMR}
///    `test_data`: a `dict` describing all of the test results
///
/// # Example
/// ```python
///    import stdfast as sf
///    raw_stdf = sf.get_raw_stdf("my_stdf.stdf")
///    raw_stdf['master_information']
/// ```
#[pyfunction]
fn get_raw_stdf(fname: &Bound<'_, PyAny>) -> PyResult<STDF> {
    let stdf = match resolve_source(fname)? {
        Source::Path(path) => STDF::from_fname(&path)?,
        Source::Bytes(bytes) => STDF::from_reader(Cursor::new(bytes))?,
    };
    Ok(stdf)
}

#[pyfunction]
fn get_mir(fname: &Bound<'_, PyAny>) -> PyResult<MIR> {
    let mir = match resolve_source(fname)? {
        Source::Path(path) => MIR::from_fname(&path)?,
        Source::Bytes(bytes) => MIR::from_reader(Cursor::new(bytes))?,
    };
    Ok(mir)
}

/// get_raw_records(fname: str)
/// --
///
/// Parse an STDF file and return a list of raw record dicts.
///
/// Each dict has a ``record_type`` key (e.g. ``"PTR"``) plus the record's fields,
/// matching the Pydantic models in ``stdfast.records``.
///
/// # Example
/// ```python
///    import stdfast as sf
///    from stdfast.records import Record
///    from pydantic import TypeAdapter
///    ta = TypeAdapter(Record)
///    records = [ta.validate_python(r) for r in sf.get_raw_records("my.stdf")]
/// ```
#[pyfunction]
fn get_raw_records(fname: &Bound<'_, PyAny>) -> PyResult<Vec<Record>> {
    let records = match resolve_source(fname)? {
        Source::Path(path) => STDF::get_raw_records_from_fname(&path),
        Source::Bytes(bytes) => STDF::get_raw_records_from_reader(Cursor::new(bytes)),
    };
    Ok(records)
}

/// Helper type representing a resolved input source.
enum Source {
    Path(String),
    Bytes(Vec<u8>),
}

/// Resolve a Python argument to either a filesystem path or raw bytes.
///
/// Accepts:
/// - `str` — treated as a file path
/// - `os.PathLike` (e.g. `pathlib.Path`) — converted via `os.fspath()` then used as a path
/// - binary file-like object (has a `.read()` method) — `.read()` is called and the bytes
///   are buffered
fn resolve_source(arg: &Bound<'_, PyAny>) -> PyResult<Source> {
    // 1. Plain str
    if let Ok(s) = arg.extract::<String>() {
        return Ok(Source::Path(s));
    }
    // 2. os.PathLike (handles pathlib.Path and anything implementing __fspath__)
    let py = arg.py();
    if let Ok(fspath_result) = py.import("os").and_then(|os| os.call_method1("fspath", (arg,))) {
        if let Ok(s) = fspath_result.extract::<String>() {
            return Ok(Source::Path(s));
        }
    }
    // 3. Binary file-like object: call .read() with no arguments
    if let Ok(result) = arg.call_method0("read") {
        if let Ok(bytes) = result.extract::<Vec<u8>>() {
            return Ok(Source::Bytes(bytes));
        }
    }
    Err(pyo3::exceptions::PyTypeError::new_err(
        "fname must be a str, os.PathLike, or a binary file-like object (with a .read() method)",
    ))
}

/// A lazy iterator over raw STDF record dicts.
///
/// Returned by ``iter_raw_records()``. Each call to ``__next__`` reads, parses,
/// and converts exactly one record from disk — only one record is live in memory
/// at a time.
#[pyclass]
pub struct RawRecordsIter {
    inner: Records<Box<dyn Read + Send + Sync>>,
}

#[pymethods]
impl RawRecordsIter {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> PyResult<Option<PyObject>> {
        let py = slf.py();
        loop {
            match slf.inner.next() {
                None => return Ok(None),
                Some(raw) => {
                    if let Some(record) = raw.resolve() {
                        let obj = record.into_pyobject(py)?.into_any().unbind();
                        return Ok(Some(obj));
                    }
                    // Unknown record type — skip silently (same as get_raw_records)
                }
            }
        }
    }
}

/// iter_raw_records(fname: str)
/// --
///
/// Return a lazy iterator over raw STDF record dicts.
///
/// Unlike ``get_raw_records()``, only one record is held in memory at a time,
/// making this suitable for files with millions of records.
///
/// Each yielded ``dict`` has a ``record_type`` key plus the record's fields.
///
/// # Example
/// ```python
///    import stdfast as sf
///    for record in sf.iter_raw_records("my.stdf"):
///        if record["record_type"] == "PTR":
///            print(record)
/// ```
#[pyfunction]
fn iter_raw_records(fname: &Bound<'_, PyAny>) -> PyResult<RawRecordsIter> {
    let inner: Records<Box<dyn Read + Send + Sync>> = match resolve_source(fname)? {
        Source::Path(path) => {
            let reader: Box<dyn Read + Send + Sync> = Box::new(BufReader::new(File::open(path)?));
            Records::from_reader(reader)
        }
        Source::Bytes(bytes) => {
            let reader: Box<dyn Read + Send + Sync> = Box::new(Cursor::new(bytes));
            Records::from_reader(reader)
        }
    };
    Ok(RawRecordsIter { inner })
}

#[pymodule]
fn stdfast(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_mir, m)?)?;
    m.add_function(wrap_pyfunction!(parse_stdf, m)?)?;
    m.add_function(wrap_pyfunction!(get_rows, m)?)?;
    m.add_function(wrap_pyfunction!(get_raw_stdf, m)?)?;
    m.add_function(wrap_pyfunction!(get_raw_records, m)?)?;
    m.add_function(wrap_pyfunction!(iter_raw_records, m)?)?;
    m.add_function(wrap_pyfunction!(crate::write_py::write_stdf, m)?)?;
    m.add_class::<crate::write_py::StdfWriter>()?;
    m.add_class::<RawRecordsIter>()?;
    Ok(())
}
