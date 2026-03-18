"""
Pydantic models for STDF record types.

Usage example::

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
"""

from __future__ import annotations

from typing import Annotated, Literal, Union

from pydantic import BaseModel, Field

# ---------------------------------------------------------------------------
# Helper type aliases matching Rust integer types
# ---------------------------------------------------------------------------
Char = Annotated[str, Field(max_length=1)]
"""A single-character string, corresponding to a Rust ``char`` field."""

U8 = Annotated[int, Field(ge=0, le=255)]
"""Rust ``u8``: unsigned 8-bit integer (0-255)."""

U16 = Annotated[int, Field(ge=0, le=65_535)]
"""Rust ``u16``: unsigned 16-bit integer (0-65535)."""

U32 = Annotated[int, Field(ge=0, le=4_294_967_295)]
"""Rust ``u32``: unsigned 32-bit integer (0-4294967295)."""

I8 = Annotated[int, Field(ge=-128, le=127)]
"""Rust ``i8``: signed 8-bit integer (-128-127)."""

I16 = Annotated[int, Field(ge=-32_768, le=32_767)]
"""Rust ``i16``: signed 16-bit integer (-32768-32767)."""

I32 = Annotated[int, Field(ge=-2_147_483_648, le=2_147_483_647)]
"""Rust ``i32``: signed 32-bit integer."""

USize = Annotated[int, Field(ge=0)]
"""Rust ``usize``: non-negative platform integer (used for byte offsets)."""

Nibble = Annotated[int, Field(ge=0, le=15)]
"""A 4-bit nibble value (0-15), used in FTR/MPR stat arrays."""


# ---------------------------------------------------------------------------
# GenData variants (used by GDR)
# ---------------------------------------------------------------------------

class GenDataU1(BaseModel):
    type: Literal["U1"] = "U1"
    value: U8


class GenDataU2(BaseModel):
    type: Literal["U2"] = "U2"
    value: U16


class GenDataU4(BaseModel):
    type: Literal["U4"] = "U4"
    value: U32


class GenDataI1(BaseModel):
    type: Literal["I1"] = "I1"
    value: I8


class GenDataI2(BaseModel):
    type: Literal["I2"] = "I2"
    value: I16


class GenDataI4(BaseModel):
    type: Literal["I4"] = "I4"
    value: I32


class GenDataR4(BaseModel):
    type: Literal["R4"] = "R4"
    value: float


class GenDataR8(BaseModel):
    type: Literal["R8"] = "R8"
    value: float


class GenDataCn(BaseModel):
    type: Literal["Cn"] = "Cn"
    value: str


class GenDataBn(BaseModel):
    """Binary data; ``value`` is a list of byte values (0–255)."""
    type: Literal["Bn"] = "Bn"
    value: list[U8]


class GenDataDn(BaseModel):
    """Bit-encoded data; ``value`` is a list of byte values (0–255)."""
    type: Literal["Dn"] = "Dn"
    value: list[U8]


class GenDataN1(BaseModel):
    """Nibble; ``value`` is 0–15."""
    type: Literal["N1"] = "N1"
    value: Nibble


GenDataItem = Annotated[
    Union[
        GenDataU1,
        GenDataU2,
        GenDataU4,
        GenDataI1,
        GenDataI2,
        GenDataI4,
        GenDataR4,
        GenDataR8,
        GenDataCn,
        GenDataBn,
        GenDataDn,
        GenDataN1,
    ],
    Field(discriminator="type"),
]
"""Discriminated union of all GenData variants."""


# ---------------------------------------------------------------------------
# STDF record models
# ---------------------------------------------------------------------------

class FAR(BaseModel):
    """File Attributes Record (REC_TYP=0, REC_SUB=10)."""
    record_type: Literal["FAR"] = "FAR"
    global_offset: USize = 0
    cpu_type: U8 = 2
    """CPU type: 1 = Sun/Motorola (big-endian), 2 = x86 (little-endian)."""
    stdf_ver: U8 = 4


class ATR(BaseModel):
    """Audit Trail Record (REC_TYP=0, REC_SUB=20)."""
    record_type: Literal["ATR"] = "ATR"
    global_offset: USize = 0
    mod_tim: U32 = 0
    cmd_line: str = ""


class MIR(BaseModel):
    """Master Information Record (REC_TYP=1, REC_SUB=10)."""
    record_type: Literal["MIR"] = "MIR"
    global_offset: USize = 0
    setup_t: U32 = 0
    start_t: U32 = 0
    stat_num: U8 = 0
    mode_cod: Literal["A", "C", "D", "E", "M", "P", "Q", " "] = " "
    rtst_cod: Char = " "
    prot_cod: Char = " "
    burn_tim: U16 = 65535
    cmod_cod: Char = " "
    lot_id: str = ""
    part_typ: str = ""
    node_nam: str = ""
    tstr_typ: str = ""
    job_nam: str = ""
    job_rev: str = ""
    sblot_id: str = ""
    oper_nam: str = ""
    exec_typ: str = ""
    exec_ver: str = ""
    test_cod: str = ""
    tst_temp: str = ""
    user_txt: str = ""
    aux_file: str = ""
    pkg_typ: str = ""
    famly_id: str = ""
    date_cod: str = ""
    facil_id: str = ""
    floor_id: str = ""
    proc_id: str = ""
    oper_frq: str = ""
    spec_nam: str = ""
    spec_ver: str = ""
    flow_id: str = ""
    setup_id: str = ""
    dsgn_rev: str = ""
    eng_id: str = ""
    rom_cod: str = ""
    serl_num: str = ""
    supr_nam: str = ""


class MRR(BaseModel):
    """Master Results Record (REC_TYP=1, REC_SUB=20)."""
    record_type: Literal["MRR"] = "MRR"
    global_offset: USize = 0
    finish_t: U32 = 0
    disp_cod: Char = " "
    usr_desc: str = ""
    exc_desc: str = ""


class SDR(BaseModel):
    """Site Description Record (REC_TYP=1, REC_SUB=80)."""
    record_type: Literal["SDR"] = "SDR"
    global_offset: USize = 0
    head_num: U8 = 1
    site_grp: U8 = 1
    site_cnt: U8 = 1
    site_num: list[U8] = Field(default_factory=lambda: [1])
    hand_typ: str = ""
    hand_id: str = ""
    card_typ: str = ""
    card_id: str = ""
    load_typ: str = ""
    load_id: str = ""
    dib_typ: str = ""
    dib_id: str = ""
    cabl_typ: str = ""
    cabl_id: str = ""
    cont_typ: str = ""
    cont_id: str = ""
    lasr_typ: str = ""
    lasr_id: str = ""
    extr_typ: str = ""
    extr_i: str = ""


class WIR(BaseModel):
    """Wafer Information Record (REC_TYP=2, REC_SUB=10)."""
    record_type: Literal["WIR"] = "WIR"
    global_offset: USize = 0
    head_num: U8 = 1
    site_grp: U8 = 255
    start_t: U32 = 0
    wafer_id: str = ""


class WRR(BaseModel):
    """Wafer Results Record (REC_TYP=2, REC_SUB=20)."""
    record_type: Literal["WRR"] = "WRR"
    global_offset: USize = 0
    head_num: U8 = 1
    site_grp: U8 = 255
    finish_t: U32 = 0
    part_cnt: U32 = 0
    rtst_cnt: U32 = 0
    abrt_cnt: U32 = 0
    good_cnt: U32 = 0
    func_cnt: U32 = 0
    wafer_id: str = ""
    fabwf_id: str = ""
    frame_id: str = ""
    mask_id: str = ""
    usr_desc: str = ""
    exc_desc: str = ""


class WCR(BaseModel):
    """Wafer Configuration Record (REC_TYP=2, REC_SUB=30)."""
    record_type: Literal["WCR"] = "WCR"
    global_offset: USize = 0
    wafr_siz: float = 0.0
    die_ht: float = 0.0
    die_wid: float = 0.0
    wf_units: U8 = 0
    wf_flat: Char = " "
    center_x: I16 = -32768
    center_y: I16 = -32768
    pos_x: Char = " "
    pos_y: Char = " "


class PIR(BaseModel):
    """Part Information Record (REC_TYP=5, REC_SUB=10)."""
    record_type: Literal["PIR"] = "PIR"
    global_offset: USize = 0
    head_num: U8 = 1
    site_num: U8 = 1


class PRR(BaseModel):
    """Part Results Record (REC_TYP=5, REC_SUB=20)."""
    record_type: Literal["PRR"] = "PRR"
    global_offset: USize = 0
    head_num: U8 = 1
    site_num: U8 = 1
    part_flg: U8 = 0
    num_test: U16 = 0
    hard_bin: U16 = 1
    soft_bin: U16 = 1
    x_coord: I16 = -32768
    y_coord: I16 = -32768
    test_t: U32 = 0
    part_id: str = ""
    part_txt: str = ""
    part_fix: list[U8] = Field(default_factory=list)


class TSR(BaseModel):
    """Test Synopsis Record (REC_TYP=10, REC_SUB=30)."""
    record_type: Literal["TSR"] = "TSR"
    global_offset: USize = 0
    head_num: U8 = 1
    site_num: U8 = 1
    test_typ: Literal[" ", "P", "F", "M"] = " "
    test_num: U32 = 0
    exec_cnt: U32 = 4294967295
    fail_cnt: U32 = 4294967295
    alrm_cnt: U32 = 4294967295
    test_nam: str = ""
    seq_name: str = ""
    test_lbl: str = ""
    opt_flag: U8 = 0xFF
    test_tim: float = 0.0
    test_min: float = 0.0
    test_max: float = 0.0
    tst_sums: float = 0.0
    tst_sqrs: float = 0.0


class PTR(BaseModel):
    """Parametric Test Record (REC_TYP=15, REC_SUB=10)."""
    record_type: Literal["PTR"] = "PTR"
    global_offset: USize = 0
    test_num: U32 = 0
    head_num: U8 = 1
    site_num: U8 = 1
    test_flg: U8 = 0
    parm_flg: U8 = 0
    result: float = 0.0
    test_txt: str = ""
    alarm_id: str = ""
    opt_flag: U8 = 0
    res_scal: I8 = 0
    llm_scal: I8 = 0
    hlm_scal: I8 = 0
    lo_limit: float = 0.0
    hi_limit: float = 0.0
    units: str = ""
    c_resfmt: str = ""
    c_llmfmt: str = ""
    c_hlmfmt: str = ""
    lo_spec: float = 0.0
    hi_spec: float = 0.0


class MPR(BaseModel):
    """Multiple-Result Parametric Record (REC_TYP=15, REC_SUB=15)."""
    record_type: Literal["MPR"] = "MPR"
    global_offset: USize = 0
    test_num: U32 = 0
    head_num: U8 = 1
    site_num: U8 = 1
    test_flg: U8 = 0
    parm_flg: U8 = 0
    rtn_icnt: U16 = 0
    rslt_cnt: U16 = 0
    rtn_stat: list[Nibble] = Field(default_factory=list)
    rtn_rslt: list[float] = Field(default_factory=list)
    test_txt: str = ""
    alarm_id: str = ""
    opt_flag: U8 = 0
    res_scal: I8 = 0
    llm_scal: I8 = 0
    hlm_scal: I8 = 0
    lo_limit: float = 0.0
    hi_limit: float = 0.0
    start_in: float = 0.0
    incr_in: float = 0.0
    rtn_indx: list[U16] = Field(default_factory=list)
    units: str = ""
    units_in: str = ""
    c_resfmt: str = ""
    c_llmfmt: str = ""
    c_hlmfmt: str = ""
    lo_spec: float = 0.0
    hi_spec: float = 0.0


class FTR(BaseModel):
    """Functional Test Record (REC_TYP=15, REC_SUB=20)."""
    record_type: Literal["FTR"] = "FTR"
    global_offset: USize = 0
    test_num: U32 = 0
    head_num: U8 = 1
    site_num: U8 = 1
    test_flg: U8 = 0
    opt_flag: U8 = 0
    cycl_cnt: U32 = 0
    rel_vadr: U32 = 0
    rept_cnt: U32 = 0
    num_fail: U32 = 0
    xfail_ad: I32 = 0
    yfail_ad: I32 = 0
    vect_off: I16 = 0
    rtn_icnt: U16 = 0
    pgm_icnt: U16 = 0
    rtn_indx: list[U16] = Field(default_factory=list)
    rtn_stat: list[Nibble] = Field(default_factory=list)
    pgm_indx: list[U16] = Field(default_factory=list)
    pgm_stat: list[Nibble] = Field(default_factory=list)
    fail_pin: list[U8] = Field(default_factory=list)
    vect_nam: str = ""
    time_set: str = ""
    op_code: str = ""
    test_txt: str = ""
    alarm_id: str = ""
    prog_txt: str = ""
    rslt_txt: str = ""
    patg_num: U8 = 255
    spin_map: list[U8] = Field(default_factory=list)


class PCR(BaseModel):
    """Part Count Record (REC_TYP=1, REC_SUB=30)."""
    record_type: Literal["PCR"] = "PCR"
    global_offset: USize = 0
    head_num: U8 = 255
    site_num: U8 = 255
    part_cnt: U32 = 0
    rtst_cnt: U32 = 4294967295
    abrt_cnt: U32 = 4294967295
    good_cnt: U32 = 4294967295
    func_cnt: U32 = 4294967295


class HBR(BaseModel):
    """Hardware Bin Record (REC_TYP=1, REC_SUB=40)."""
    record_type: Literal["HBR"] = "HBR"
    global_offset: USize = 0
    head_num: U8 = 1
    site_num: U8 = 1
    hbin_num: U16 = 0
    hbin_cnt: U32 = 0
    hbin_pf: Literal["P", "F", " "] = " "
    hbin_nam: str = ""


class SBR(BaseModel):
    """Software Bin Record (REC_TYP=1, REC_SUB=50)."""
    record_type: Literal["SBR"] = "SBR"
    global_offset: USize = 0
    head_num: U8 = 1
    site_num: U8 = 1
    sbin_num: U16 = 0
    sbin_cnt: U32 = 0
    sbin_pf: Literal["P", "F", " "] = " "
    sbin_nam: str = ""


class PMR(BaseModel):
    """Pin Map Record (REC_TYP=1, REC_SUB=60)."""
    record_type: Literal["PMR"] = "PMR"
    global_offset: USize = 0
    pmr_indx: U16 = 1
    chan_typ: U16 = 0
    chan_nam: str = ""
    phy_nam: str = ""
    log_nam: str = ""
    head_num: U8 = 1
    site_num: U8 = 1


class PGR(BaseModel):
    """Pin Group Record (REC_TYP=1, REC_SUB=62)."""
    record_type: Literal["PGR"] = "PGR"
    global_offset: USize = 0
    grp_indx: U16 = 0
    grp_nam: str = ""
    indx_cnt: U16 = 0
    pmr_indx: list[U16] = Field(default_factory=list)


class PLR(BaseModel):
    """Pin List Record (REC_TYP=1, REC_SUB=63)."""
    record_type: Literal["PLR"] = "PLR"
    global_offset: USize = 0
    grp_cnt: U16 = 0
    grp_indx: list[U16] = Field(default_factory=list)
    grp_mode: list[U16] = Field(default_factory=list)
    grp_radx: list[U8] = Field(default_factory=list)
    pgm_char: list[str] = Field(default_factory=list)
    rtn_char: list[str] = Field(default_factory=list)
    pgm_chal: list[str] = Field(default_factory=list)
    rtn_chal: list[str] = Field(default_factory=list)


class RDR(BaseModel):
    """Retest Data Record (REC_TYP=1, REC_SUB=70)."""
    record_type: Literal["RDR"] = "RDR"
    global_offset: USize = 0
    num_bins: U16 = 0
    rtst_bin: list[U16] = Field(default_factory=list)


class BPS(BaseModel):
    """Begin Program Section Record (REC_TYP=20, REC_SUB=10)."""
    record_type: Literal["BPS"] = "BPS"
    global_offset: USize = 0
    seq_name: str = ""


class EPS(BaseModel):
    """End Program Section Record (REC_TYP=20, REC_SUB=20). Contains no data."""
    record_type: Literal["EPS"] = "EPS"
    global_offset: USize = 0


class GDR(BaseModel):
    """Generic Data Record (REC_TYP=50, REC_SUB=10)."""
    record_type: Literal["GDR"] = "GDR"
    global_offset: USize = 0
    fld_cnt: U16 = 0
    gen_data: list[GenDataItem] = Field(default_factory=list)


class DTR(BaseModel):
    """Datalog Text Record (REC_TYP=50, REC_SUB=30)."""
    record_type: Literal["DTR"] = "DTR"
    global_offset: USize = 0
    text_dat: str = ""


# ---------------------------------------------------------------------------
# Discriminated union of all record types
# ---------------------------------------------------------------------------

Record = Annotated[
    Union[
        FAR,
        ATR,
        MIR,
        MRR,
        PCR,
        HBR,
        SBR,
        PMR,
        PGR,
        PLR,
        RDR,
        SDR,
        WIR,
        WRR,
        WCR,
        PIR,
        PRR,
        TSR,
        PTR,
        MPR,
        FTR,
        BPS,
        EPS,
        GDR,
        DTR,
    ],
    Field(discriminator="record_type"),
]
"""Discriminated union of all STDF record types, keyed by ``record_type``."""
