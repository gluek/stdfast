//! Python bindings for writing STDF files.
//!
//! Implements [`pyo3::FromPyObject`] for each record type so that Pydantic model
//! instances produced by `stdfast.records` can be passed directly from Python.
//!
//! # Example
//! ```python
//! import stdfast as sf
//! from stdfast.records import FAR, MIR, MRR, PIR, PTR, PRR
//!
//! records = [
//!     FAR(cpu_type=2, stdf_ver=4),
//!     MIR(lot_id="LOT001", part_typ="MYPART", job_nam="MYJOB"),
//!     PIR(head_num=1, site_num=1),
//!     PTR(test_num=1000, head_num=1, site_num=1, result=1.23, test_txt="my_test"),
//!     PRR(head_num=1, site_num=1, hard_bin=1, soft_bin=1, num_test=1),
//!     MRR(),
//! ]
//! sf.write_stdf("output.stdf", records)
//! ```

use pyo3::prelude::*;

use crate::{
    records::record_impl::*,
    util::GenData,
};

// ---------------------------------------------------------------------------
// Helper: extract a single char from a Python str attribute
// ---------------------------------------------------------------------------

fn extract_char(ob: &Bound<'_, PyAny>, field: &str) -> PyResult<char> {
    let s: String = ob.getattr(field)?.extract()?;
    Ok(s.chars().next().unwrap_or(' '))
}

// ---------------------------------------------------------------------------
// GenData
// ---------------------------------------------------------------------------

impl<'py> FromPyObject<'py> for GenData {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let type_str: String = ob.getattr("type")?.extract()?;
        let value = ob.getattr("value")?;
        match type_str.as_str() {
            "U1" => Ok(GenData::U1(value.extract()?)),
            "U2" => Ok(GenData::U2(value.extract()?)),
            "U4" => Ok(GenData::U4(value.extract()?)),
            "I1" => Ok(GenData::I1(value.extract()?)),
            "I2" => Ok(GenData::I2(value.extract()?)),
            "I4" => Ok(GenData::I4(value.extract()?)),
            "R4" => Ok(GenData::R4(value.extract()?)),
            "R8" => Ok(GenData::R8(value.extract()?)),
            "Cn" => Ok(GenData::Cn(value.extract()?)),
            "Bn" => Ok(GenData::Bn(value.extract()?)),
            "Dn" => Ok(GenData::Dn(value.extract()?)),
            "N1" => Ok(GenData::N1(value.extract()?)),
            other => Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Unknown GenData type: '{other}'"
            ))),
        }
    }
}

// ---------------------------------------------------------------------------
// FromPyObject for every record struct
// ---------------------------------------------------------------------------

impl<'py> FromPyObject<'py> for FAR {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self {
            global_offset: ob.getattr("global_offset").and_then(|x| x.extract()).unwrap_or(0),
            cpu_type: ob.getattr("cpu_type")?.extract()?,
            stdf_ver: ob.getattr("stdf_ver")?.extract()?,
        })
    }
}

impl<'py> FromPyObject<'py> for ATR {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self {
            global_offset: ob.getattr("global_offset").and_then(|x| x.extract()).unwrap_or(0),
            mod_tim: ob.getattr("mod_tim")?.extract()?,
            cmd_line: ob.getattr("cmd_line")?.extract()?,
        })
    }
}

impl<'py> FromPyObject<'py> for MIR {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self {
            global_offset: ob.getattr("global_offset").and_then(|x| x.extract()).unwrap_or(0),
            setup_t: ob.getattr("setup_t")?.extract()?,
            start_t: ob.getattr("start_t")?.extract()?,
            stat_num: ob.getattr("stat_num")?.extract()?,
            mode_cod: extract_char(ob, "mode_cod")?,
            rtst_cod: extract_char(ob, "rtst_cod")?,
            prot_cod: extract_char(ob, "prot_cod")?,
            burn_tim: ob.getattr("burn_tim")?.extract()?,
            cmod_cod: extract_char(ob, "cmod_cod")?,
            lot_id: ob.getattr("lot_id")?.extract()?,
            part_typ: ob.getattr("part_typ")?.extract()?,
            node_nam: ob.getattr("node_nam")?.extract()?,
            tstr_typ: ob.getattr("tstr_typ")?.extract()?,
            job_nam: ob.getattr("job_nam")?.extract()?,
            job_rev: ob.getattr("job_rev")?.extract()?,
            sblot_id: ob.getattr("sblot_id")?.extract()?,
            oper_nam: ob.getattr("oper_nam")?.extract()?,
            exec_typ: ob.getattr("exec_typ")?.extract()?,
            exec_ver: ob.getattr("exec_ver")?.extract()?,
            test_cod: ob.getattr("test_cod")?.extract()?,
            tst_temp: ob.getattr("tst_temp")?.extract()?,
            user_txt: ob.getattr("user_txt")?.extract()?,
            aux_file: ob.getattr("aux_file")?.extract()?,
            pkg_typ: ob.getattr("pkg_typ")?.extract()?,
            famly_id: ob.getattr("famly_id")?.extract()?,
            date_cod: ob.getattr("date_cod")?.extract()?,
            facil_id: ob.getattr("facil_id")?.extract()?,
            floor_id: ob.getattr("floor_id")?.extract()?,
            proc_id: ob.getattr("proc_id")?.extract()?,
            oper_frq: ob.getattr("oper_frq")?.extract()?,
            spec_nam: ob.getattr("spec_nam")?.extract()?,
            spec_ver: ob.getattr("spec_ver")?.extract()?,
            flow_id: ob.getattr("flow_id")?.extract()?,
            setup_id: ob.getattr("setup_id")?.extract()?,
            dsgn_rev: ob.getattr("dsgn_rev")?.extract()?,
            eng_id: ob.getattr("eng_id")?.extract()?,
            rom_cod: ob.getattr("rom_cod")?.extract()?,
            serl_num: ob.getattr("serl_num")?.extract()?,
            supr_nam: ob.getattr("supr_nam")?.extract()?,
        })
    }
}

impl<'py> FromPyObject<'py> for MRR {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self {
            global_offset: ob.getattr("global_offset").and_then(|x| x.extract()).unwrap_or(0),
            finish_t: ob.getattr("finish_t")?.extract()?,
            disp_cod: extract_char(ob, "disp_cod")?,
            usr_desc: ob.getattr("usr_desc")?.extract()?,
            exc_desc: ob.getattr("exc_desc")?.extract()?,
        })
    }
}

impl<'py> FromPyObject<'py> for SDR {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self {
            global_offset: ob.getattr("global_offset").and_then(|x| x.extract()).unwrap_or(0),
            head_num: ob.getattr("head_num")?.extract()?,
            site_grp: ob.getattr("site_grp")?.extract()?,
            site_cnt: ob.getattr("site_cnt")?.extract()?,
            site_num: ob.getattr("site_num")?.extract()?,
            hand_typ: ob.getattr("hand_typ")?.extract()?,
            hand_id: ob.getattr("hand_id")?.extract()?,
            card_typ: ob.getattr("card_typ")?.extract()?,
            card_id: ob.getattr("card_id")?.extract()?,
            load_typ: ob.getattr("load_typ")?.extract()?,
            load_id: ob.getattr("load_id")?.extract()?,
            dib_typ: ob.getattr("dib_typ")?.extract()?,
            dib_id: ob.getattr("dib_id")?.extract()?,
            cabl_typ: ob.getattr("cabl_typ")?.extract()?,
            cabl_id: ob.getattr("cabl_id")?.extract()?,
            cont_typ: ob.getattr("cont_typ")?.extract()?,
            cont_id: ob.getattr("cont_id")?.extract()?,
            lasr_typ: ob.getattr("lasr_typ")?.extract()?,
            lasr_id: ob.getattr("lasr_id")?.extract()?,
            extr_typ: ob.getattr("extr_typ")?.extract()?,
            extr_i: ob.getattr("extr_i")?.extract()?,
        })
    }
}

impl<'py> FromPyObject<'py> for WIR {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self {
            global_offset: ob.getattr("global_offset").and_then(|x| x.extract()).unwrap_or(0),
            head_num: ob.getattr("head_num")?.extract()?,
            site_grp: ob.getattr("site_grp")?.extract()?,
            start_t: ob.getattr("start_t")?.extract()?,
            wafer_id: ob.getattr("wafer_id")?.extract()?,
        })
    }
}

impl<'py> FromPyObject<'py> for WRR {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self {
            global_offset: ob.getattr("global_offset").and_then(|x| x.extract()).unwrap_or(0),
            head_num: ob.getattr("head_num")?.extract()?,
            site_grp: ob.getattr("site_grp")?.extract()?,
            finish_t: ob.getattr("finish_t")?.extract()?,
            part_cnt: ob.getattr("part_cnt")?.extract()?,
            rtst_cnt: ob.getattr("rtst_cnt")?.extract()?,
            abrt_cnt: ob.getattr("abrt_cnt")?.extract()?,
            good_cnt: ob.getattr("good_cnt")?.extract()?,
            func_cnt: ob.getattr("func_cnt")?.extract()?,
            wafer_id: ob.getattr("wafer_id")?.extract()?,
            fabwf_id: ob.getattr("fabwf_id")?.extract()?,
            frame_id: ob.getattr("frame_id")?.extract()?,
            mask_id: ob.getattr("mask_id")?.extract()?,
            usr_desc: ob.getattr("usr_desc")?.extract()?,
            exc_desc: ob.getattr("exc_desc")?.extract()?,
        })
    }
}

impl<'py> FromPyObject<'py> for WCR {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self {
            global_offset: ob.getattr("global_offset").and_then(|x| x.extract()).unwrap_or(0),
            wafr_siz: ob.getattr("wafr_siz")?.extract()?,
            die_ht: ob.getattr("die_ht")?.extract()?,
            die_wid: ob.getattr("die_wid")?.extract()?,
            wf_units: ob.getattr("wf_units")?.extract()?,
            wf_flat: extract_char(ob, "wf_flat")?,
            center_x: ob.getattr("center_x")?.extract()?,
            center_y: ob.getattr("center_y")?.extract()?,
            pos_x: extract_char(ob, "pos_x")?,
            pos_y: extract_char(ob, "pos_y")?,
        })
    }
}

impl<'py> FromPyObject<'py> for PIR {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self {
            global_offset: ob.getattr("global_offset").and_then(|x| x.extract()).unwrap_or(0),
            head_num: ob.getattr("head_num")?.extract()?,
            site_num: ob.getattr("site_num")?.extract()?,
        })
    }
}

impl<'py> FromPyObject<'py> for PRR {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self {
            global_offset: ob.getattr("global_offset").and_then(|x| x.extract()).unwrap_or(0),
            head_num: ob.getattr("head_num")?.extract()?,
            site_num: ob.getattr("site_num")?.extract()?,
            part_flg: ob.getattr("part_flg")?.extract()?,
            num_test: ob.getattr("num_test")?.extract()?,
            hard_bin: ob.getattr("hard_bin")?.extract()?,
            soft_bin: ob.getattr("soft_bin")?.extract()?,
            x_coord: ob.getattr("x_coord")?.extract()?,
            y_coord: ob.getattr("y_coord")?.extract()?,
            test_t: ob.getattr("test_t")?.extract()?,
            part_id: ob.getattr("part_id")?.extract()?,
            part_txt: ob.getattr("part_txt")?.extract()?,
            part_fix: ob.getattr("part_fix")?.extract()?,
        })
    }
}

impl<'py> FromPyObject<'py> for PCR {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self {
            global_offset: ob.getattr("global_offset").and_then(|x| x.extract()).unwrap_or(0),
            head_num: ob.getattr("head_num")?.extract()?,
            site_num: ob.getattr("site_num")?.extract()?,
            part_cnt: ob.getattr("part_cnt")?.extract()?,
            rtst_cnt: ob.getattr("rtst_cnt")?.extract()?,
            abrt_cnt: ob.getattr("abrt_cnt")?.extract()?,
            good_cnt: ob.getattr("good_cnt")?.extract()?,
            func_cnt: ob.getattr("func_cnt")?.extract()?,
        })
    }
}

impl<'py> FromPyObject<'py> for HBR {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self {
            global_offset: ob.getattr("global_offset").and_then(|x| x.extract()).unwrap_or(0),
            head_num: ob.getattr("head_num")?.extract()?,
            site_num: ob.getattr("site_num")?.extract()?,
            hbin_num: ob.getattr("hbin_num")?.extract()?,
            hbin_cnt: ob.getattr("hbin_cnt")?.extract()?,
            hbin_pf: extract_char(ob, "hbin_pf")?,
            hbin_nam: ob.getattr("hbin_nam")?.extract()?,
        })
    }
}

impl<'py> FromPyObject<'py> for SBR {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self {
            global_offset: ob.getattr("global_offset").and_then(|x| x.extract()).unwrap_or(0),
            head_num: ob.getattr("head_num")?.extract()?,
            site_num: ob.getattr("site_num")?.extract()?,
            sbin_num: ob.getattr("sbin_num")?.extract()?,
            sbin_cnt: ob.getattr("sbin_cnt")?.extract()?,
            sbin_pf: extract_char(ob, "sbin_pf")?,
            sbin_nam: ob.getattr("sbin_nam")?.extract()?,
        })
    }
}

impl<'py> FromPyObject<'py> for PMR {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self {
            global_offset: ob.getattr("global_offset").and_then(|x| x.extract()).unwrap_or(0),
            pmr_indx: ob.getattr("pmr_indx")?.extract()?,
            chan_typ: ob.getattr("chan_typ")?.extract()?,
            chan_nam: ob.getattr("chan_nam")?.extract()?,
            phy_nam: ob.getattr("phy_nam")?.extract()?,
            log_nam: ob.getattr("log_nam")?.extract()?,
            head_num: ob.getattr("head_num")?.extract()?,
            site_num: ob.getattr("site_num")?.extract()?,
        })
    }
}

impl<'py> FromPyObject<'py> for PGR {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self {
            global_offset: ob.getattr("global_offset").and_then(|x| x.extract()).unwrap_or(0),
            grp_indx: ob.getattr("grp_indx")?.extract()?,
            grp_nam: ob.getattr("grp_nam")?.extract()?,
            indx_cnt: ob.getattr("indx_cnt")?.extract()?,
            pmr_indx: ob.getattr("pmr_indx")?.extract()?,
        })
    }
}

impl<'py> FromPyObject<'py> for PLR {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self {
            global_offset: ob.getattr("global_offset").and_then(|x| x.extract()).unwrap_or(0),
            grp_cnt: ob.getattr("grp_cnt")?.extract()?,
            grp_indx: ob.getattr("grp_indx")?.extract()?,
            grp_mode: ob.getattr("grp_mode")?.extract()?,
            grp_radx: ob.getattr("grp_radx")?.extract()?,
            pgm_char: ob.getattr("pgm_char")?.extract()?,
            rtn_char: ob.getattr("rtn_char")?.extract()?,
            pgm_chal: ob.getattr("pgm_chal")?.extract()?,
            rtn_chal: ob.getattr("rtn_chal")?.extract()?,
        })
    }
}

impl<'py> FromPyObject<'py> for RDR {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self {
            global_offset: ob.getattr("global_offset").and_then(|x| x.extract()).unwrap_or(0),
            num_bins: ob.getattr("num_bins")?.extract()?,
            rtst_bin: ob.getattr("rtst_bin")?.extract()?,
        })
    }
}

impl<'py> FromPyObject<'py> for TSR {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self {
            global_offset: ob.getattr("global_offset").and_then(|x| x.extract()).unwrap_or(0),
            head_num: ob.getattr("head_num")?.extract()?,
            site_num: ob.getattr("site_num")?.extract()?,
            test_typ: extract_char(ob, "test_typ")?,
            test_num: ob.getattr("test_num")?.extract()?,
            exec_cnt: ob.getattr("exec_cnt")?.extract()?,
            fail_cnt: ob.getattr("fail_cnt")?.extract()?,
            alrm_cnt: ob.getattr("alrm_cnt")?.extract()?,
            test_nam: ob.getattr("test_nam")?.extract()?,
            seq_name: ob.getattr("seq_name")?.extract()?,
            test_lbl: ob.getattr("test_lbl")?.extract()?,
            opt_flag: ob.getattr("opt_flag")?.extract()?,
            test_tim: ob.getattr("test_tim")?.extract()?,
            test_min: ob.getattr("test_min")?.extract()?,
            test_max: ob.getattr("test_max")?.extract()?,
            tst_sums: ob.getattr("tst_sums")?.extract()?,
            tst_sqrs: ob.getattr("tst_sqrs")?.extract()?,
        })
    }
}

impl<'py> FromPyObject<'py> for PTR {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self {
            global_offset: ob.getattr("global_offset").and_then(|x| x.extract()).unwrap_or(0),
            test_num: ob.getattr("test_num")?.extract()?,
            head_num: ob.getattr("head_num")?.extract()?,
            site_num: ob.getattr("site_num")?.extract()?,
            test_flg: ob.getattr("test_flg")?.extract()?,
            parm_flg: ob.getattr("parm_flg")?.extract()?,
            result: ob.getattr("result")?.extract()?,
            test_txt: ob.getattr("test_txt")?.extract()?,
            alarm_id: ob.getattr("alarm_id")?.extract()?,
            opt_flag: ob.getattr("opt_flag")?.extract()?,
            res_scal: ob.getattr("res_scal")?.extract()?,
            llm_scal: ob.getattr("llm_scal")?.extract()?,
            hlm_scal: ob.getattr("hlm_scal")?.extract()?,
            lo_limit: ob.getattr("lo_limit")?.extract()?,
            hi_limit: ob.getattr("hi_limit")?.extract()?,
            units: ob.getattr("units")?.extract()?,
            c_resfmt: ob.getattr("c_resfmt")?.extract()?,
            c_llmfmt: ob.getattr("c_llmfmt")?.extract()?,
            c_hlmfmt: ob.getattr("c_hlmfmt")?.extract()?,
            lo_spec: ob.getattr("lo_spec")?.extract()?,
            hi_spec: ob.getattr("hi_spec")?.extract()?,
        })
    }
}

impl<'py> FromPyObject<'py> for MPR {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self {
            global_offset: ob.getattr("global_offset").and_then(|x| x.extract()).unwrap_or(0),
            test_num: ob.getattr("test_num")?.extract()?,
            head_num: ob.getattr("head_num")?.extract()?,
            site_num: ob.getattr("site_num")?.extract()?,
            test_flg: ob.getattr("test_flg")?.extract()?,
            parm_flg: ob.getattr("parm_flg")?.extract()?,
            rtn_icnt: ob.getattr("rtn_icnt")?.extract()?,
            rslt_cnt: ob.getattr("rslt_cnt")?.extract()?,
            rtn_stat: ob.getattr("rtn_stat")?.extract()?,
            rtn_rslt: ob.getattr("rtn_rslt")?.extract()?,
            test_txt: ob.getattr("test_txt")?.extract()?,
            alarm_id: ob.getattr("alarm_id")?.extract()?,
            opt_flag: ob.getattr("opt_flag")?.extract()?,
            res_scal: ob.getattr("res_scal")?.extract()?,
            llm_scal: ob.getattr("llm_scal")?.extract()?,
            hlm_scal: ob.getattr("hlm_scal")?.extract()?,
            lo_limit: ob.getattr("lo_limit")?.extract()?,
            hi_limit: ob.getattr("hi_limit")?.extract()?,
            start_in: ob.getattr("start_in")?.extract()?,
            incr_in: ob.getattr("incr_in")?.extract()?,
            rtn_indx: ob.getattr("rtn_indx")?.extract()?,
            units: ob.getattr("units")?.extract()?,
            units_in: ob.getattr("units_in")?.extract()?,
            c_resfmt: ob.getattr("c_resfmt")?.extract()?,
            c_llmfmt: ob.getattr("c_llmfmt")?.extract()?,
            c_hlmfmt: ob.getattr("c_hlmfmt")?.extract()?,
            lo_spec: ob.getattr("lo_spec")?.extract()?,
            hi_spec: ob.getattr("hi_spec")?.extract()?,
        })
    }
}

impl<'py> FromPyObject<'py> for FTR {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self {
            global_offset: ob.getattr("global_offset").and_then(|x| x.extract()).unwrap_or(0),
            test_num: ob.getattr("test_num")?.extract()?,
            head_num: ob.getattr("head_num")?.extract()?,
            site_num: ob.getattr("site_num")?.extract()?,
            test_flg: ob.getattr("test_flg")?.extract()?,
            opt_flag: ob.getattr("opt_flag")?.extract()?,
            cycl_cnt: ob.getattr("cycl_cnt")?.extract()?,
            rel_vadr: ob.getattr("rel_vadr")?.extract()?,
            rept_cnt: ob.getattr("rept_cnt")?.extract()?,
            num_fail: ob.getattr("num_fail")?.extract()?,
            xfail_ad: ob.getattr("xfail_ad")?.extract()?,
            yfail_ad: ob.getattr("yfail_ad")?.extract()?,
            vect_off: ob.getattr("vect_off")?.extract()?,
            rtn_icnt: ob.getattr("rtn_icnt")?.extract()?,
            pgm_icnt: ob.getattr("pgm_icnt")?.extract()?,
            rtn_indx: ob.getattr("rtn_indx")?.extract()?,
            rtn_stat: ob.getattr("rtn_stat")?.extract()?,
            pgm_indx: ob.getattr("pgm_indx")?.extract()?,
            pgm_stat: ob.getattr("pgm_stat")?.extract()?,
            fail_pin: ob.getattr("fail_pin")?.extract()?,
            vect_nam: ob.getattr("vect_nam")?.extract()?,
            time_set: ob.getattr("time_set")?.extract()?,
            op_code: ob.getattr("op_code")?.extract()?,
            test_txt: ob.getattr("test_txt")?.extract()?,
            alarm_id: ob.getattr("alarm_id")?.extract()?,
            prog_txt: ob.getattr("prog_txt")?.extract()?,
            rslt_txt: ob.getattr("rslt_txt")?.extract()?,
            patg_num: ob.getattr("patg_num")?.extract()?,
            spin_map: ob.getattr("spin_map")?.extract()?,
        })
    }
}

impl<'py> FromPyObject<'py> for BPS {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self {
            global_offset: ob.getattr("global_offset").and_then(|x| x.extract()).unwrap_or(0),
            seq_name: ob.getattr("seq_name")?.extract()?,
        })
    }
}

impl<'py> FromPyObject<'py> for EPS {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self {
            global_offset: ob.getattr("global_offset").and_then(|x| x.extract()).unwrap_or(0),
            dummy_field: String::new(),
        })
    }
}

impl<'py> FromPyObject<'py> for GDR {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self {
            global_offset: ob.getattr("global_offset").and_then(|x| x.extract()).unwrap_or(0),
            fld_cnt: ob.getattr("fld_cnt")?.extract()?,
            gen_data: ob.getattr("gen_data")?.extract()?,
        })
    }
}

impl<'py> FromPyObject<'py> for DTR {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self {
            global_offset: ob.getattr("global_offset").and_then(|x| x.extract()).unwrap_or(0),
            text_dat: ob.getattr("text_dat")?.extract()?,
        })
    }
}

// ---------------------------------------------------------------------------
// write_stdf Python function
// ---------------------------------------------------------------------------

/// write_stdf(fname: str, records: list)
/// --
///
/// Serialize a list of STDF record objects to a binary STDF file.
///
/// `fname` must be a `str` path to the output file (will be created or overwritten).
/// `records` is a list of record model instances from `stdfast.records` (e.g. `FAR`,
/// `MIR`, `PTR`, …).  Each object must have a `record_type` attribute matching the
/// class name.
///
/// # Example
/// ```python
///    from stdfast.records import FAR, MIR, MRR, PIR, PTR, PRR
///    import stdfast as sf
///
///    records = [
///        FAR(cpu_type=2, stdf_ver=4),
///        MIR(lot_id="LOT001", part_typ="MY_PART"),
///        PIR(head_num=1, site_num=1),
///        PTR(test_num=1000, head_num=1, site_num=1, result=3.14, test_txt="vdd"),
///        PRR(head_num=1, site_num=1, hard_bin=1, soft_bin=1, num_test=1),
///        MRR(),
///    ]
///    sf.write_stdf("out.stdf", records)
/// ```
#[pyfunction]
pub fn write_stdf(fname: &str, records: Vec<Bound<'_, PyAny>>) -> PyResult<()> {
    let mut bytes: Vec<u8> = Vec::new();

    for record in &records {
        let record_type: String = record.getattr("record_type")?.extract()?;
        let record_bytes: Vec<u8> = match record_type.as_str() {
            "FAR" => { let r: FAR = record.extract()?; r.bytes() }
            "ATR" => { let r: ATR = record.extract()?; r.bytes() }
            "MIR" => { let r: MIR = record.extract()?; r.bytes() }
            "MRR" => { let r: MRR = record.extract()?; r.bytes() }
            "SDR" => { let r: SDR = record.extract()?; r.bytes() }
            "WIR" => { let r: WIR = record.extract()?; r.bytes() }
            "WRR" => { let r: WRR = record.extract()?; r.bytes() }
            "WCR" => { let r: WCR = record.extract()?; r.bytes() }
            "PIR" => { let r: PIR = record.extract()?; r.bytes() }
            "PRR" => { let r: PRR = record.extract()?; r.bytes() }
            "PCR" => { let r: PCR = record.extract()?; r.bytes() }
            "HBR" => { let r: HBR = record.extract()?; r.bytes() }
            "SBR" => { let r: SBR = record.extract()?; r.bytes() }
            "PMR" => { let r: PMR = record.extract()?; r.bytes() }
            "PGR" => { let r: PGR = record.extract()?; r.bytes() }
            "PLR" => { let r: PLR = record.extract()?; r.bytes() }
            "RDR" => { let r: RDR = record.extract()?; r.bytes() }
            "TSR" => { let r: TSR = record.extract()?; r.bytes() }
            "PTR" => { let r: PTR = record.extract()?; r.bytes() }
            "MPR" => { let r: MPR = record.extract()?; r.bytes() }
            "FTR" => { let r: FTR = record.extract()?; r.bytes() }
            "BPS" => { let r: BPS = record.extract()?; r.bytes() }
            "EPS" => { let r: EPS = record.extract()?; r.bytes() }
            "GDR" => { let r: GDR = record.extract()?; r.bytes() }
            "DTR" => { let r: DTR = record.extract()?; r.bytes() }
            other => {
                return Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "Unknown record_type: '{other}'"
                )));
            }
        };
        bytes.extend(record_bytes);
    }

    std::fs::write(fname, bytes)?;
    Ok(())
}
