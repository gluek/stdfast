use stdfast::record_types::RecordType;
use stdfast::records::{
    Header, RawRecord,
    record_impl::*,
};
use stdfast::records::record_impl::GenData;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Reconstruct a `RawRecord` from a full serialised record (header + contents).
fn bytes_to_raw(bytes: &[u8]) -> RawRecord {
    let header_bytes: &[u8; 4] = bytes[..4].try_into().unwrap();
    let header = Header::from_bytes(header_bytes);
    let rtype = RecordType::new(header.rec_typ, header.rec_sub);
    let contents = bytes[4..].to_vec();
    RawRecord {
        header,
        offset: 0,
        contents,
        rtype,
    }
}

/// Bit-exact f32 comparison (NaN-safe, avoids float rounding surprises).
fn assert_f32_bits_eq(a: f32, b: f32) {
    assert_eq!(
        a.to_bits(),
        b.to_bits(),
        "f32 mismatch: {a} vs {b} (bits: {:08X} vs {:08X})",
        a.to_bits(),
        b.to_bits()
    );
}

// ---------------------------------------------------------------------------
// FAR – File Attributes Record
// ---------------------------------------------------------------------------

#[test]
fn test_far_roundtrip() {
    let original = FAR {
        global_offset: 0,
        cpu_type: 2,
        stdf_ver: 4,
    };
    let bytes = original.bytes();
    let raw = bytes_to_raw(&bytes);
    let parsed = FAR::from(&raw);

    assert_eq!(parsed.cpu_type, original.cpu_type);
    assert_eq!(parsed.stdf_ver, original.stdf_ver);
}

// ---------------------------------------------------------------------------
// PIR – Part Information Record
// ---------------------------------------------------------------------------

#[test]
fn test_pir_roundtrip() {
    let original = PIR {
        global_offset: 0,
        head_num: 1,
        site_num: 3,
    };
    let bytes = original.bytes();
    let raw = bytes_to_raw(&bytes);
    let parsed = PIR::from(&raw);

    assert_eq!(parsed.head_num, original.head_num);
    assert_eq!(parsed.site_num, original.site_num);
}

// ---------------------------------------------------------------------------
// PRR – Part Results Record
// ---------------------------------------------------------------------------

#[test]
fn test_prr_roundtrip() {
    let original = PRR {
        global_offset: 0,
        head_num: 1,
        site_num: 2,
        part_flg: 0,
        num_test: 5,
        hard_bin: 1,
        soft_bin: 2,
        x_coord: -10,
        y_coord: 20,
        test_t: 1234,
        part_id: "PART001".to_string(),
        part_txt: "some part".to_string(),
        part_fix: vec![0xAB, 0xCD],
    };
    let bytes = original.bytes();
    let raw = bytes_to_raw(&bytes);
    let parsed = PRR::from(&raw);

    assert_eq!(parsed.head_num, original.head_num);
    assert_eq!(parsed.site_num, original.site_num);
    assert_eq!(parsed.part_flg, original.part_flg);
    assert_eq!(parsed.num_test, original.num_test);
    assert_eq!(parsed.hard_bin, original.hard_bin);
    assert_eq!(parsed.soft_bin, original.soft_bin);
    assert_eq!(parsed.x_coord, original.x_coord);
    assert_eq!(parsed.y_coord, original.y_coord);
    assert_eq!(parsed.test_t, original.test_t);
    assert_eq!(parsed.part_id, original.part_id);
    assert_eq!(parsed.part_txt, original.part_txt);
    assert_eq!(parsed.part_fix, original.part_fix);
}

// ---------------------------------------------------------------------------
// PTR – Parametric Test Record
// ---------------------------------------------------------------------------

#[test]
fn test_ptr_roundtrip() {
    let original = PTR {
        global_offset: 0,
        test_num: 1001,
        head_num: 1,
        site_num: 0,
        test_flg: 0x00,
        parm_flg: 0x00,
        result: 1.23456,
        test_txt: "VDD_current".to_string(),
        alarm_id: "".to_string(),
        opt_flag: 0x00,
        res_scal: -3,
        llm_scal: -3,
        hlm_scal: -3,
        lo_limit: 0.5,
        hi_limit: 2.0,
        units: "mA".to_string(),
        c_resfmt: "%7.3f".to_string(),
        c_llmfmt: "%7.3f".to_string(),
        c_hlmfmt: "%7.3f".to_string(),
        lo_spec: 0.0,
        hi_spec: 5.0,
    };
    let bytes = original.bytes();
    let raw = bytes_to_raw(&bytes);
    let parsed = PTR::from(&raw);

    assert_eq!(parsed.test_num, original.test_num);
    assert_eq!(parsed.head_num, original.head_num);
    assert_eq!(parsed.site_num, original.site_num);
    assert_eq!(parsed.test_flg, original.test_flg);
    assert_eq!(parsed.parm_flg, original.parm_flg);
    assert_f32_bits_eq(parsed.result, original.result);
    assert_eq!(parsed.test_txt, original.test_txt);
    assert_eq!(parsed.alarm_id, original.alarm_id);
    assert_eq!(parsed.opt_flag, original.opt_flag);
    assert_eq!(parsed.res_scal, original.res_scal);
    assert_eq!(parsed.llm_scal, original.llm_scal);
    assert_eq!(parsed.hlm_scal, original.hlm_scal);
    assert_f32_bits_eq(parsed.lo_limit, original.lo_limit);
    assert_f32_bits_eq(parsed.hi_limit, original.hi_limit);
    assert_eq!(parsed.units, original.units);
    assert_eq!(parsed.c_resfmt, original.c_resfmt);
    assert_eq!(parsed.c_llmfmt, original.c_llmfmt);
    assert_eq!(parsed.c_hlmfmt, original.c_hlmfmt);
    assert_f32_bits_eq(parsed.lo_spec, original.lo_spec);
    assert_f32_bits_eq(parsed.hi_spec, original.hi_spec);
}

// ---------------------------------------------------------------------------
// FTR – Functional Test Record
// ---------------------------------------------------------------------------

#[test]
fn test_ftr_roundtrip() {
    // Even rtn_icnt and pgm_icnt keep nibble arrays length-stable across round-trips.
    let original = FTR {
        global_offset: 0,
        test_num: 2001,
        head_num: 1,
        site_num: 0,
        test_flg: 0x00,
        opt_flag: 0xFF,
        cycl_cnt: 100,
        rel_vadr: 0,
        rept_cnt: 1,
        num_fail: 0,
        xfail_ad: -1,
        yfail_ad: -2,
        vect_off: 0,
        rtn_icnt: 4,
        pgm_icnt: 2,
        rtn_indx: vec![1, 2, 3, 4],
        rtn_stat: vec![0xA, 0x5, 0x3, 0x7],
        pgm_indx: vec![4, 5],
        pgm_stat: vec![0x1, 0x2],
        fail_pin: vec![0xDE, 0xAD],
        vect_nam: "VEC_A".to_string(),
        time_set: "T1".to_string(),
        op_code: "".to_string(),
        test_txt: "func_test_1".to_string(),
        alarm_id: "".to_string(),
        prog_txt: "".to_string(),
        rslt_txt: "PASS".to_string(),
        patg_num: 7,
        spin_map: vec![0xFF],
    };
    let bytes = original.bytes();
    let raw = bytes_to_raw(&bytes);
    let parsed = FTR::from(&raw);

    assert_eq!(parsed.test_num, original.test_num);
    assert_eq!(parsed.head_num, original.head_num);
    assert_eq!(parsed.site_num, original.site_num);
    assert_eq!(parsed.test_flg, original.test_flg);
    assert_eq!(parsed.opt_flag, original.opt_flag);
    assert_eq!(parsed.cycl_cnt, original.cycl_cnt);
    assert_eq!(parsed.rept_cnt, original.rept_cnt);
    assert_eq!(parsed.num_fail, original.num_fail);
    assert_eq!(parsed.xfail_ad, original.xfail_ad);
    assert_eq!(parsed.yfail_ad, original.yfail_ad);
    assert_eq!(parsed.vect_off, original.vect_off);
    assert_eq!(parsed.rtn_icnt, original.rtn_icnt);
    assert_eq!(parsed.pgm_icnt, original.pgm_icnt);
    assert_eq!(parsed.rtn_indx, original.rtn_indx);
    assert_eq!(parsed.rtn_stat, original.rtn_stat);
    assert_eq!(parsed.pgm_indx, original.pgm_indx);
    assert_eq!(parsed.pgm_stat, original.pgm_stat);
    assert_eq!(parsed.fail_pin, original.fail_pin);
    assert_eq!(parsed.vect_nam, original.vect_nam);
    assert_eq!(parsed.time_set, original.time_set);
    assert_eq!(parsed.op_code, original.op_code);
    assert_eq!(parsed.test_txt, original.test_txt);
    assert_eq!(parsed.alarm_id, original.alarm_id);
    assert_eq!(parsed.prog_txt, original.prog_txt);
    assert_eq!(parsed.rslt_txt, original.rslt_txt);
    assert_eq!(parsed.patg_num, original.patg_num);
    assert_eq!(parsed.spin_map, original.spin_map);
}

/// With an odd `rtn_icnt`, the last nibble byte carries a zero-padded upper nibble.
/// `kxN1` always unpacks both nibbles from each byte, so the parsed vector is one element
/// longer than the original. The first `rtn_icnt` elements are identical; the extra element
/// is zero.
#[test]
fn test_ftr_nibble_odd_padding() {
    let original = FTR {
        global_offset: 0,
        test_num: 2002,
        head_num: 1,
        site_num: 0,
        test_flg: 0x00,
        opt_flag: 0x00,
        cycl_cnt: 0,
        rel_vadr: 0,
        rept_cnt: 0,
        num_fail: 0,
        xfail_ad: 0,
        yfail_ad: 0,
        vect_off: 0,
        rtn_icnt: 3,
        pgm_icnt: 0,
        rtn_indx: vec![1, 2, 3],
        rtn_stat: vec![0xA, 0x5, 0x3],
        pgm_indx: vec![],
        pgm_stat: vec![],
        fail_pin: vec![],
        vect_nam: "".to_string(),
        time_set: "".to_string(),
        op_code: "".to_string(),
        test_txt: "".to_string(),
        alarm_id: "".to_string(),
        prog_txt: "".to_string(),
        rslt_txt: "".to_string(),
        patg_num: 0,
        spin_map: vec![],
    };
    let bytes = original.bytes();
    let raw = bytes_to_raw(&bytes);
    let parsed = FTR::from(&raw);

    // The parser unpacks both nibbles of every packed byte, so an odd rtn_icnt produces
    // one extra element (the zero-padded upper nibble of the last byte).
    assert_eq!(parsed.rtn_stat.len(), 4);
    assert_eq!(&parsed.rtn_stat[..3], &original.rtn_stat[..]);
    assert_eq!(parsed.rtn_stat[3], 0);
}

// ---------------------------------------------------------------------------
// MPR – Multiple-Result Parametric Record
// ---------------------------------------------------------------------------

#[test]
fn test_mpr_roundtrip() {
    let original = MPR {
        global_offset: 0,
        test_num: 3001,
        head_num: 1,
        site_num: 0,
        test_flg: 0x00,
        parm_flg: 0x00,
        rtn_icnt: 4,
        rslt_cnt: 4,
        rtn_stat: vec![0x1, 0x2, 0x3, 0x4],
        rtn_rslt: vec![1.0, 2.5, 3.14, 0.001],
        test_txt: "continuity".to_string(),
        alarm_id: "".to_string(),
        opt_flag: 0x00,
        res_scal: 0,
        llm_scal: 0,
        hlm_scal: 0,
        lo_limit: -0.5,
        hi_limit: 0.5,
        start_in: 0.0,
        incr_in: 0.1,
        rtn_indx: vec![1, 2, 3, 4],
        units: "V".to_string(),
        units_in: "V".to_string(),
        c_resfmt: "%6.3f".to_string(),
        c_llmfmt: "%6.3f".to_string(),
        c_hlmfmt: "%6.3f".to_string(),
        lo_spec: -1.0,
        hi_spec: 1.0,
    };
    let bytes = original.bytes();
    let raw = bytes_to_raw(&bytes);
    let parsed = MPR::from(&raw);

    assert_eq!(parsed.test_num, original.test_num);
    assert_eq!(parsed.head_num, original.head_num);
    assert_eq!(parsed.site_num, original.site_num);
    assert_eq!(parsed.test_flg, original.test_flg);
    assert_eq!(parsed.parm_flg, original.parm_flg);
    assert_eq!(parsed.rtn_icnt, original.rtn_icnt);
    assert_eq!(parsed.rslt_cnt, original.rslt_cnt);
    assert_eq!(parsed.rtn_stat, original.rtn_stat);
    assert_eq!(parsed.rtn_rslt.len(), original.rtn_rslt.len());
    for (a, b) in parsed.rtn_rslt.iter().zip(original.rtn_rslt.iter()) {
        assert_f32_bits_eq(*a, *b);
    }
    assert_eq!(parsed.test_txt, original.test_txt);
    assert_eq!(parsed.alarm_id, original.alarm_id);
    assert_eq!(parsed.opt_flag, original.opt_flag);
    assert_eq!(parsed.res_scal, original.res_scal);
    assert_eq!(parsed.llm_scal, original.llm_scal);
    assert_eq!(parsed.hlm_scal, original.hlm_scal);
    assert_f32_bits_eq(parsed.lo_limit, original.lo_limit);
    assert_f32_bits_eq(parsed.hi_limit, original.hi_limit);
    assert_f32_bits_eq(parsed.start_in, original.start_in);
    assert_f32_bits_eq(parsed.incr_in, original.incr_in);
    assert_eq!(parsed.rtn_indx, original.rtn_indx);
    assert_eq!(parsed.units, original.units);
    assert_eq!(parsed.units_in, original.units_in);
    assert_eq!(parsed.c_resfmt, original.c_resfmt);
    assert_eq!(parsed.c_llmfmt, original.c_llmfmt);
    assert_eq!(parsed.c_hlmfmt, original.c_hlmfmt);
    assert_f32_bits_eq(parsed.lo_spec, original.lo_spec);
    assert_f32_bits_eq(parsed.hi_spec, original.hi_spec);
}

// ---------------------------------------------------------------------------
// ATR – Audit Trail Record
// ---------------------------------------------------------------------------

#[test]
fn test_atr_roundtrip() {
    let original = ATR {
        global_offset: 0,
        mod_tim: 1700000000,
        cmd_line: "stdfast --convert input.stdf".to_string(),
    };
    let bytes = original.bytes();
    let raw = bytes_to_raw(&bytes);
    let parsed = ATR::from(&raw);

    assert_eq!(parsed.mod_tim, original.mod_tim);
    assert_eq!(parsed.cmd_line, original.cmd_line);
}

// ---------------------------------------------------------------------------
// MIR – Master Information Record
// ---------------------------------------------------------------------------

#[test]
fn test_mir_roundtrip() {
    let original = MIR {
        global_offset: 0,
        setup_t: 1700000000,
        start_t: 1700000100,
        stat_num: 3,
        mode_cod: 'P',
        rtst_cod: ' ',
        prot_cod: ' ',
        burn_tim: 0,
        cmod_cod: ' ',
        lot_id: "LOT001".to_string(),
        part_typ: "CHIP_XYZ".to_string(),
        node_nam: "TESTER01".to_string(),
        tstr_typ: "TESTSYS".to_string(),
        job_nam: "MYJOB".to_string(),
        job_rev: "1.0".to_string(),
        sblot_id: "SUB01".to_string(),
        oper_nam: "JOHNDOE".to_string(),
        exec_typ: "EXEC".to_string(),
        exec_ver: "2.5".to_string(),
        test_cod: "FC".to_string(),
        tst_temp: "25C".to_string(),
        user_txt: "test comment".to_string(),
        aux_file: "aux.stdf".to_string(),
        pkg_typ: "QFP".to_string(),
        famly_id: "FAM01".to_string(),
        date_cod: "20231001".to_string(),
        facil_id: "FAB1".to_string(),
        floor_id: "F01".to_string(),
        proc_id: "PROC1".to_string(),
        oper_frq: "100MHz".to_string(),
        spec_nam: "SPEC1".to_string(),
        spec_ver: "1.0".to_string(),
        flow_id: "FLOW1".to_string(),
        setup_id: "SETUP1".to_string(),
        dsgn_rev: "A".to_string(),
        eng_id: "ENG01".to_string(),
        rom_cod: "ROM1".to_string(),
        serl_num: "SN001".to_string(),
        supr_nam: "SUPERVISOR".to_string(),
    };
    let bytes = original.bytes();
    let raw = bytes_to_raw(&bytes);
    let parsed = MIR::from(&raw);

    assert_eq!(parsed.setup_t, original.setup_t);
    assert_eq!(parsed.start_t, original.start_t);
    assert_eq!(parsed.stat_num, original.stat_num);
    assert_eq!(parsed.mode_cod, original.mode_cod);
    assert_eq!(parsed.rtst_cod, original.rtst_cod);
    assert_eq!(parsed.prot_cod, original.prot_cod);
    assert_eq!(parsed.burn_tim, original.burn_tim);
    assert_eq!(parsed.cmod_cod, original.cmod_cod);
    assert_eq!(parsed.lot_id, original.lot_id);
    assert_eq!(parsed.part_typ, original.part_typ);
    assert_eq!(parsed.node_nam, original.node_nam);
    assert_eq!(parsed.tstr_typ, original.tstr_typ);
    assert_eq!(parsed.job_nam, original.job_nam);
    assert_eq!(parsed.job_rev, original.job_rev);
    assert_eq!(parsed.sblot_id, original.sblot_id);
    assert_eq!(parsed.oper_nam, original.oper_nam);
    assert_eq!(parsed.exec_typ, original.exec_typ);
    assert_eq!(parsed.exec_ver, original.exec_ver);
    assert_eq!(parsed.test_cod, original.test_cod);
    assert_eq!(parsed.tst_temp, original.tst_temp);
    assert_eq!(parsed.user_txt, original.user_txt);
    assert_eq!(parsed.aux_file, original.aux_file);
    assert_eq!(parsed.pkg_typ, original.pkg_typ);
    assert_eq!(parsed.famly_id, original.famly_id);
    assert_eq!(parsed.date_cod, original.date_cod);
    assert_eq!(parsed.facil_id, original.facil_id);
    assert_eq!(parsed.floor_id, original.floor_id);
    assert_eq!(parsed.proc_id, original.proc_id);
    assert_eq!(parsed.oper_frq, original.oper_frq);
    assert_eq!(parsed.spec_nam, original.spec_nam);
    assert_eq!(parsed.spec_ver, original.spec_ver);
    assert_eq!(parsed.flow_id, original.flow_id);
    assert_eq!(parsed.setup_id, original.setup_id);
    assert_eq!(parsed.dsgn_rev, original.dsgn_rev);
    assert_eq!(parsed.eng_id, original.eng_id);
    assert_eq!(parsed.rom_cod, original.rom_cod);
    assert_eq!(parsed.serl_num, original.serl_num);
    assert_eq!(parsed.supr_nam, original.supr_nam);
}

// ---------------------------------------------------------------------------
// MRR – Master Results Record
// ---------------------------------------------------------------------------

#[test]
fn test_mrr_roundtrip() {
    let original = MRR {
        global_offset: 0,
        finish_t: 1700001000,
        disp_cod: 'P',
        usr_desc: "All tests passed".to_string(),
        exc_desc: "".to_string(),
    };
    let bytes = original.bytes();
    let raw = bytes_to_raw(&bytes);
    let parsed = MRR::from(&raw);

    assert_eq!(parsed.finish_t, original.finish_t);
    assert_eq!(parsed.disp_cod, original.disp_cod);
    assert_eq!(parsed.usr_desc, original.usr_desc);
    assert_eq!(parsed.exc_desc, original.exc_desc);
}

// ---------------------------------------------------------------------------
// PCR – Part Count Record
// ---------------------------------------------------------------------------

#[test]
fn test_pcr_roundtrip() {
    let original = PCR {
        global_offset: 0,
        head_num: 255,
        site_num: 255,
        part_cnt: 1000,
        rtst_cnt: 5,
        abrt_cnt: 2,
        good_cnt: 993,
        func_cnt: 998,
    };
    let bytes = original.bytes();
    let raw = bytes_to_raw(&bytes);
    let parsed = PCR::from(&raw);

    assert_eq!(parsed.head_num, original.head_num);
    assert_eq!(parsed.site_num, original.site_num);
    assert_eq!(parsed.part_cnt, original.part_cnt);
    assert_eq!(parsed.rtst_cnt, original.rtst_cnt);
    assert_eq!(parsed.abrt_cnt, original.abrt_cnt);
    assert_eq!(parsed.good_cnt, original.good_cnt);
    assert_eq!(parsed.func_cnt, original.func_cnt);
}

// ---------------------------------------------------------------------------
// HBR – Hardware Bin Record
// ---------------------------------------------------------------------------

#[test]
fn test_hbr_roundtrip() {
    let original = HBR {
        global_offset: 0,
        head_num: 1,
        site_num: 0,
        hbin_num: 1,
        hbin_cnt: 500,
        hbin_pf: 'P',
        hbin_nam: "PASS".to_string(),
    };
    let bytes = original.bytes();
    let raw = bytes_to_raw(&bytes);
    let parsed = HBR::from(&raw);

    assert_eq!(parsed.head_num, original.head_num);
    assert_eq!(parsed.site_num, original.site_num);
    assert_eq!(parsed.hbin_num, original.hbin_num);
    assert_eq!(parsed.hbin_cnt, original.hbin_cnt);
    assert_eq!(parsed.hbin_pf, original.hbin_pf);
    assert_eq!(parsed.hbin_nam, original.hbin_nam);
}

// ---------------------------------------------------------------------------
// SBR – Software Bin Record
// ---------------------------------------------------------------------------

#[test]
fn test_sbr_roundtrip() {
    let original = SBR {
        global_offset: 0,
        head_num: 1,
        site_num: 0,
        sbin_num: 10,
        sbin_cnt: 487,
        sbin_pf: 'F',
        sbin_nam: "FAIL_CURRENT".to_string(),
    };
    let bytes = original.bytes();
    let raw = bytes_to_raw(&bytes);
    let parsed = SBR::from(&raw);

    assert_eq!(parsed.head_num, original.head_num);
    assert_eq!(parsed.site_num, original.site_num);
    assert_eq!(parsed.sbin_num, original.sbin_num);
    assert_eq!(parsed.sbin_cnt, original.sbin_cnt);
    assert_eq!(parsed.sbin_pf, original.sbin_pf);
    assert_eq!(parsed.sbin_nam, original.sbin_nam);
}

// ---------------------------------------------------------------------------
// PMR – Pin Map Record
// ---------------------------------------------------------------------------

#[test]
fn test_pmr_roundtrip() {
    let original = PMR {
        global_offset: 0,
        pmr_indx: 42,
        chan_typ: 1,
        chan_nam: "CH_A".to_string(),
        phy_nam: "VDD".to_string(),
        log_nam: "VDD_LOGIC".to_string(),
        head_num: 1,
        site_num: 0,
    };
    let bytes = original.bytes();
    let raw = bytes_to_raw(&bytes);
    let parsed = PMR::from(&raw);

    assert_eq!(parsed.pmr_indx, original.pmr_indx);
    assert_eq!(parsed.chan_typ, original.chan_typ);
    assert_eq!(parsed.chan_nam, original.chan_nam);
    assert_eq!(parsed.phy_nam, original.phy_nam);
    assert_eq!(parsed.log_nam, original.log_nam);
    assert_eq!(parsed.head_num, original.head_num);
    assert_eq!(parsed.site_num, original.site_num);
}

// ---------------------------------------------------------------------------
// PGR – Pin Group Record
// ---------------------------------------------------------------------------

#[test]
fn test_pgr_roundtrip() {
    let original = PGR {
        global_offset: 0,
        grp_indx: 100,
        grp_nam: "POWER_PINS".to_string(),
        indx_cnt: 3,
        pmr_indx: vec![1, 2, 3],
    };
    let bytes = original.bytes();
    let raw = bytes_to_raw(&bytes);
    let parsed = PGR::from(&raw);

    assert_eq!(parsed.grp_indx, original.grp_indx);
    assert_eq!(parsed.grp_nam, original.grp_nam);
    assert_eq!(parsed.indx_cnt, original.indx_cnt);
    assert_eq!(parsed.pmr_indx, original.pmr_indx);
}

// ---------------------------------------------------------------------------
// PLR – Pin List Record
// ---------------------------------------------------------------------------

#[test]
fn test_plr_roundtrip() {
    let original = PLR {
        global_offset: 0,
        grp_cnt: 2,
        grp_indx: vec![100, 101],
        grp_mode: vec![0x0200, 0x0200],
        grp_radx: vec![2, 16],
        pgm_char: vec!["01".to_string(), "FF".to_string()],
        rtn_char: vec!["ZZ".to_string(), "HH".to_string()],
        pgm_chal: vec!["AB".to_string(), "CD".to_string()],
        rtn_chal: vec!["EF".to_string(), "GH".to_string()],
    };
    let bytes = original.bytes();
    let raw = bytes_to_raw(&bytes);
    let parsed = PLR::from(&raw);

    assert_eq!(parsed.grp_cnt, original.grp_cnt);
    assert_eq!(parsed.grp_indx, original.grp_indx);
    assert_eq!(parsed.grp_mode, original.grp_mode);
    assert_eq!(parsed.grp_radx, original.grp_radx);
    assert_eq!(parsed.pgm_char, original.pgm_char);
    assert_eq!(parsed.rtn_char, original.rtn_char);
    assert_eq!(parsed.pgm_chal, original.pgm_chal);
    assert_eq!(parsed.rtn_chal, original.rtn_chal);
}

// ---------------------------------------------------------------------------
// RDR – Retest Data Record
// ---------------------------------------------------------------------------

#[test]
fn test_rdr_roundtrip() {
    let original = RDR {
        global_offset: 0,
        num_bins: 4,
        rtst_bin: vec![1, 5, 10, 255],
    };
    let bytes = original.bytes();
    let raw = bytes_to_raw(&bytes);
    let parsed = RDR::from(&raw);

    assert_eq!(parsed.num_bins, original.num_bins);
    assert_eq!(parsed.rtst_bin, original.rtst_bin);
}

// ---------------------------------------------------------------------------
// SDR – Site Description Record
// ---------------------------------------------------------------------------

#[test]
fn test_sdr_roundtrip() {
    let original = SDR {
        global_offset: 0,
        head_num: 1,
        site_grp: 0,
        site_cnt: 3,
        site_num: vec![0, 1, 2],
        hand_typ: "HANDLER_A".to_string(),
        hand_id: "H001".to_string(),
        card_typ: "CARD_X".to_string(),
        card_id: "C001".to_string(),
        load_typ: "LOAD_T".to_string(),
        load_id: "L001".to_string(),
        dib_typ: "DIB_T".to_string(),
        dib_id: "D001".to_string(),
        cabl_typ: "CABLE_T".to_string(),
        cabl_id: "CAB001".to_string(),
        cont_typ: "CONT_T".to_string(),
        cont_id: "CON001".to_string(),
        lasr_typ: "LASR_T".to_string(),
        lasr_id: "LAS001".to_string(),
        extr_typ: "EXTR_T".to_string(),
        extr_i: "EXTRA1".to_string(),
    };
    let bytes = original.bytes();
    let raw = bytes_to_raw(&bytes);
    let parsed = SDR::from(&raw);

    assert_eq!(parsed.head_num, original.head_num);
    assert_eq!(parsed.site_grp, original.site_grp);
    assert_eq!(parsed.site_cnt, original.site_cnt);
    assert_eq!(parsed.site_num, original.site_num);
    assert_eq!(parsed.hand_typ, original.hand_typ);
    assert_eq!(parsed.hand_id, original.hand_id);
    assert_eq!(parsed.card_typ, original.card_typ);
    assert_eq!(parsed.card_id, original.card_id);
    assert_eq!(parsed.load_typ, original.load_typ);
    assert_eq!(parsed.load_id, original.load_id);
    assert_eq!(parsed.dib_typ, original.dib_typ);
    assert_eq!(parsed.dib_id, original.dib_id);
    assert_eq!(parsed.cabl_typ, original.cabl_typ);
    assert_eq!(parsed.cabl_id, original.cabl_id);
    assert_eq!(parsed.cont_typ, original.cont_typ);
    assert_eq!(parsed.cont_id, original.cont_id);
    assert_eq!(parsed.lasr_typ, original.lasr_typ);
    assert_eq!(parsed.lasr_id, original.lasr_id);
    assert_eq!(parsed.extr_typ, original.extr_typ);
    assert_eq!(parsed.extr_i, original.extr_i);
}

// ---------------------------------------------------------------------------
// WIR – Wafer Information Record
// ---------------------------------------------------------------------------

#[test]
fn test_wir_roundtrip() {
    let original = WIR {
        global_offset: 0,
        head_num: 1,
        site_grp: 255,
        start_t: 1700000200,
        wafer_id: "WAFER_007".to_string(),
    };
    let bytes = original.bytes();
    let raw = bytes_to_raw(&bytes);
    let parsed = WIR::from(&raw);

    assert_eq!(parsed.head_num, original.head_num);
    assert_eq!(parsed.site_grp, original.site_grp);
    assert_eq!(parsed.start_t, original.start_t);
    assert_eq!(parsed.wafer_id, original.wafer_id);
}

// ---------------------------------------------------------------------------
// WRR – Wafer Results Record
// ---------------------------------------------------------------------------

#[test]
fn test_wrr_roundtrip() {
    let original = WRR {
        global_offset: 0,
        head_num: 1,
        site_grp: 255,
        finish_t: 1700003600,
        part_cnt: 2000,
        rtst_cnt: 10,
        abrt_cnt: 3,
        good_cnt: 1987,
        func_cnt: 1997,
        wafer_id: "WAFER_007".to_string(),
        fabwf_id: "FW_007".to_string(),
        frame_id: "FRAME_1".to_string(),
        mask_id: "MASK_A".to_string(),
        usr_desc: "production lot".to_string(),
        exc_desc: "".to_string(),
    };
    let bytes = original.bytes();
    let raw = bytes_to_raw(&bytes);
    let parsed = WRR::from(&raw);

    assert_eq!(parsed.head_num, original.head_num);
    assert_eq!(parsed.site_grp, original.site_grp);
    assert_eq!(parsed.finish_t, original.finish_t);
    assert_eq!(parsed.part_cnt, original.part_cnt);
    assert_eq!(parsed.rtst_cnt, original.rtst_cnt);
    assert_eq!(parsed.abrt_cnt, original.abrt_cnt);
    assert_eq!(parsed.good_cnt, original.good_cnt);
    assert_eq!(parsed.func_cnt, original.func_cnt);
    assert_eq!(parsed.wafer_id, original.wafer_id);
    assert_eq!(parsed.fabwf_id, original.fabwf_id);
    assert_eq!(parsed.frame_id, original.frame_id);
    assert_eq!(parsed.mask_id, original.mask_id);
    assert_eq!(parsed.usr_desc, original.usr_desc);
    assert_eq!(parsed.exc_desc, original.exc_desc);
}

// ---------------------------------------------------------------------------
// WCR – Wafer Configuration Record
// ---------------------------------------------------------------------------

#[test]
fn test_wcr_roundtrip() {
    let original = WCR {
        global_offset: 0,
        wafr_siz: 300.0,
        die_ht: 5.5,
        die_wid: 4.2,
        wf_units: 4,
        wf_flat: 'U',
        center_x: -1,
        center_y: -1,
        pos_x: 'R',
        pos_y: 'U',
    };
    let bytes = original.bytes();
    let raw = bytes_to_raw(&bytes);
    let parsed = WCR::from(&raw);

    assert_f32_bits_eq(parsed.wafr_siz, original.wafr_siz);
    assert_f32_bits_eq(parsed.die_ht, original.die_ht);
    assert_f32_bits_eq(parsed.die_wid, original.die_wid);
    assert_eq!(parsed.wf_units, original.wf_units);
    assert_eq!(parsed.wf_flat, original.wf_flat);
    assert_eq!(parsed.center_x, original.center_x);
    assert_eq!(parsed.center_y, original.center_y);
    assert_eq!(parsed.pos_x, original.pos_x);
    assert_eq!(parsed.pos_y, original.pos_y);
}

// ---------------------------------------------------------------------------
// TSR – Test Synopsis Record
// ---------------------------------------------------------------------------

#[test]
fn test_tsr_roundtrip() {
    let original = TSR {
        global_offset: 0,
        head_num: 1,
        site_num: 0,
        test_typ: 'P',
        test_num: 1001,
        exec_cnt: 500,
        fail_cnt: 3,
        alrm_cnt: 0,
        test_nam: "VDD_Current".to_string(),
        seq_name: "SEQ1".to_string(),
        test_lbl: "LABEL1".to_string(),
        opt_flag: 0x00,
        test_tim: 0.001,
        test_min: 0.5,
        test_max: 1.5,
        tst_sums: 500.0,
        tst_sqrs: 5000.0,
    };
    let bytes = original.bytes();
    let raw = bytes_to_raw(&bytes);
    let parsed = TSR::from(&raw);

    assert_eq!(parsed.head_num, original.head_num);
    assert_eq!(parsed.site_num, original.site_num);
    assert_eq!(parsed.test_typ, original.test_typ);
    assert_eq!(parsed.test_num, original.test_num);
    assert_eq!(parsed.exec_cnt, original.exec_cnt);
    assert_eq!(parsed.fail_cnt, original.fail_cnt);
    assert_eq!(parsed.alrm_cnt, original.alrm_cnt);
    assert_eq!(parsed.test_nam, original.test_nam);
    assert_eq!(parsed.seq_name, original.seq_name);
    assert_eq!(parsed.test_lbl, original.test_lbl);
    assert_eq!(parsed.opt_flag, original.opt_flag);
    assert_f32_bits_eq(parsed.test_tim, original.test_tim);
    assert_f32_bits_eq(parsed.test_min, original.test_min);
    assert_f32_bits_eq(parsed.test_max, original.test_max);
    assert_f32_bits_eq(parsed.tst_sums, original.tst_sums);
    assert_f32_bits_eq(parsed.tst_sqrs, original.tst_sqrs);
}

// ---------------------------------------------------------------------------
// BPS – Begin Program Section Record
// ---------------------------------------------------------------------------

#[test]
fn test_bps_roundtrip() {
    let original = BPS {
        global_offset: 0,
        seq_name: "section_alpha".to_string(),
    };
    let bytes = original.bytes();
    let raw = bytes_to_raw(&bytes);
    let parsed = BPS::from(&raw);

    assert_eq!(parsed.seq_name, original.seq_name);
}

// ---------------------------------------------------------------------------
// EPS – End Program Section Record
// ---------------------------------------------------------------------------

#[test]
fn test_eps_roundtrip() {
    // EPS carries no data; its bytes() always produces the same 4-byte record.
    let original_bytes: Vec<u8> = {
        let rec_len: i16 = 0;
        [rec_len.to_ne_bytes().as_ref(), &[20u8, 20u8]].concat()
    };
    let raw = bytes_to_raw(&original_bytes);
    let parsed = EPS::from(&raw);
    assert_eq!(parsed.bytes(), original_bytes);
}

// ---------------------------------------------------------------------------
// DTR – Datalog Text Record
// ---------------------------------------------------------------------------

#[test]
fn test_dtr_roundtrip() {
    let original = DTR {
        global_offset: 0,
        text_dat: "This is a datalog entry with special chars: <>/&".to_string(),
    };
    let bytes = original.bytes();
    let raw = bytes_to_raw(&bytes);
    let parsed = DTR::from(&raw);

    assert_eq!(parsed.text_dat, original.text_dat);
}

// ---------------------------------------------------------------------------
// GDR – Generic Data Record
// ---------------------------------------------------------------------------

#[test]
fn test_gdr_roundtrip() {
    let original = GDR {
        global_offset: 0,
        fld_cnt: 8,
        gen_data: vec![
            GenData::U1(42),
            GenData::U2(1000),
            GenData::U4(100000),
            GenData::I1(-10),
            GenData::I2(-500),
            GenData::I4(-100000),
            GenData::Cn("hello".to_string()),
            GenData::N1(7),
        ],
    };
    let bytes = original.bytes();
    let raw = bytes_to_raw(&bytes);
    let parsed = GDR::from(&raw);

    // Re-serializing the parsed record must produce identical bytes.
    assert_eq!(parsed.bytes(), bytes);
    assert_eq!(parsed.fld_cnt, original.fld_cnt);
    assert_eq!(parsed.gen_data.len(), original.gen_data.len());
}
