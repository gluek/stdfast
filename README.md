# stdfast
This is a fork from [stupidf](https://github.com/jlazear/stupidf), jlazear

`stdfast` is a library for parsing and writing of STDF files. The `STDF` structure can be used
directly in rust, or alternatively sent out to Python. The library exposes several functions to read and write STDF file records from python. Additionally, there is a pydantic model for each record type to simplify the handling in python.

STDF is the [Standard Test Data Format](https://en.wikipedia.org/wiki/Standard_Test_Data_Format) and is commonly used for high-volume test of semiconductors in Automated Test Equipment (ATE) systems. 

The purpose of the library is to quickly and efficiently parse and write STDF files (which are a fairly unfriendly binary linked list-based format).


# Examples

## Python

Also contains Python bindings to this functionality, e.g.

### Parsing

**Full parse into a dict of DataFrames (eager):**
```python
import stdfast as sf

stdf = sf.parse_stdf("my_stdf.stdf")
stdf["data"]              # polars DataFrame of all test results
stdf["test_information"]  # DataFrame of test metadata
stdf["master_information"]["lot_id"]  # MIR field
```

**Raw record dicts, fully loaded (eager):**
```python
import stdfast as sf

records = sf.get_raw_records("my_stdf.stdf")
ptrs = [r for r in records if r["record_type"] == "PTR"]
```

**Pydantic models, fully loaded (eager):**
```python
import stdfast as sf

records = sf.get_records("my_stdf.stdf")
failing = [r for r in records if r.record_type == "PTR" and not r.pass_()]
```

**Lazy iteration over raw dicts — memory-efficient for large files:**
```python
import stdfast as sf

for record in sf.iter_raw_records("my_stdf.stdf"):
    if record["record_type"] == "PTR":
        print(record["test_num"], record["result"])
```

**Lazy iteration over Pydantic models:**
```python
import stdfast as sf

for record in sf.iter_records("my_stdf.stdf"):
    if record.record_type == "PTR":
        print(record.test_num, record.result)
```

**Collect only MIR (file header metadata):**
```python
import stdfast as sf

mir = sf.get_mir("my_stdf.stdf")
print(mir["lot_id"], mir["part_typ"])
```

### Writing

**Batch write with `write_stdf` (builds the full list in memory first):**
```python
import stdfast as sf
from stdfast.records import FAR, MIR, MRR, PIR, PTR, PRR

records = [
    FAR(cpu_type=2, stdf_ver=4),
    MIR(lot_id="LOT001", part_typ="MYPART", job_nam="MYJOB"),
    PIR(head_num=1, site_num=1),
    PTR(test_num=1000, head_num=1, site_num=1, result=1.23, test_txt="my_test"),
    PRR(head_num=1, site_num=1, hard_bin=1, soft_bin=1, num_test=1),
    MRR(),
]
sf.write_stdf("output.stdf", records)
```

**Streaming write with `StdfWriter` (flushes each record immediately):**
```python
import stdfast as sf
from stdfast.records import FAR, MIR, MRR, PIR, PTR, PRR

# Creates new file or overwrites content if it exists
with sf.StdfWriter("output.stdf", append=False) as w:
    w.write_record(FAR(cpu_type=2, stdf_ver=4))
    w.write_record(MIR(lot_id="LOT001", part_typ="MYPART", job_nam="MYJOB"))
    for i, result in enumerate(my_results):
        w.write_record(PIR(head_num=1, site_num=1))
        w.write_record(PTR(test_num=1000 + i, head_num=1, site_num=1, result=result, test_txt=f"test_{i}"))
        w.write_record(PRR(head_num=1, site_num=1, hard_bin=1, soft_bin=1, num_test=1))
    w.write_record(MRR())

# Creates new file or appends to content if it exists
with sf.StdfWriter("output.stdf", append=True) as w:
    ...

```

`StdfWriter` is preferable when the number of records is large or unknown upfront, since it never holds more than one record in memory at a time.

## Rust

### Parsing

```rust
use stdfast::data::STDF;
use polars::prelude::*;

let verbose = false;
if let Ok(stdf) = STDF::from_fname(&fname, verbose) {
    let df: DataFrame = (&stdf.test_data).into();
    let df_fmti: DataFrame = (&stdf.test_data.test_information).into();
    println!("{df:#?}");
    println!("{df_fmti}");
    }
```

`Records` is a lazy iterator over `RawRecord`s. Each `RawRecord` reads from the file but does not
parse its contents until `.resolve()` is called, which returns an `Option<Record>`. This lets you
skip or filter records without paying the parsing cost for every one.

**Print every record as ATDF text:**
```rust
use stdfast::records::Records;

let records = Records::new("my_stdf.stdf")?;
for raw in records {
    if let Some(record) = raw.resolve() {
        println!("{record}");
    }
}
```

**Collect only PTR records, resolving nothing else:**
```rust
use stdfast::records::{Records, record_impl::Record};

let ptrs: Vec<_> = Records::new("my_stdf.stdf")?
    .filter_map(|raw| raw.resolve())
    .filter_map(|r| if let Record::PTR(ptr) = r { Some(ptr) } else { None })
    .collect();
```

**Count records by type without resolving any:**
```rust
use stdfast::records::{Records, RecordSummary};

let mut summary = RecordSummary::new();
for raw in Records::new("my_stdf.stdf")? {
    summary.add(&raw);
}
for (rtype, count) in summary {
    println!("{rtype:?}: {count}");
}
```

**Find the first failing PTR:**
```rust
use stdfast::records::{Records, record_impl::Record};

let first_fail = Records::new("my_stdf.stdf")?
    .filter_map(|raw| raw.resolve())
    .filter_map(|r| if let Record::PTR(ptr) = r { Some(ptr) } else { None })
    .find(|ptr| !ptr.pass());
```


# Installation

There is no cargo or pypi release yet.

## Building from source

Rust:

```bash
cargo build
```

Python bindings (requires an active virtualenv):

```
pip install maturin
maturin develop
```

# Development

If you're seeing issues with `pyo3` recompiling on every build, even when there are no `pyo3`-related changes, then you're most likely running into [this issue](https://github.com/PyO3/pyo3/issues/1708). 

Consider setting the `PYO3_PYTHON` env variable adding to your `Cargo.toml` or terminal:

```
[env]
PYO3_PYTHON = /path/to/python
```

and also ensuring this is the Python interpreter used by your IDE. E.g. if using nvim, activate the venv before starting nvim. 
