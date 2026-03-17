use super::Records;
use crate::records::RawRecord;
use crate::util::*;
pub use crate::util::GenData;
use pyo3::prelude::IntoPyObject;
use std::{io, fmt};

/// File Attributes Record
#[derive(Debug, IntoPyObject, Clone)]
#[allow(dead_code)]
pub struct FAR {
    pub cpu_type: u8,
    pub stdf_ver: u8,
}

impl From<&RawRecord> for FAR {
    fn from(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let mut offset: usize = 0;
        let cpu_type = U1(contents, &mut offset);
        let stdf_ver = U1(contents, &mut offset);

        Self {
            cpu_type,
            stdf_ver
        }
    }
}

impl fmt::Display for FAR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FAR:{}|{}", self.cpu_type, self.stdf_ver)
    }
}

impl FAR {
    pub fn bytes(&self) -> Vec<u8> {
        let rec_len: i16 = 2;
        let rec_typ_sub: &[u8] = &[0u8, 10u8];
        [
            &rec_len.to_ne_bytes(),
            rec_typ_sub,
            &self.cpu_type.to_ne_bytes(),
            &self.stdf_ver.to_ne_bytes()
        ].concat()
    }
}

/// Audit Trail Record
#[derive(Debug, IntoPyObject, Clone)]
#[allow(dead_code)]
pub struct ATR {
    pub mod_tim: u32,
    pub cmd_line: String,
}

impl From<&RawRecord> for ATR {
    fn from(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let mut offset: usize = 0;
        let mod_tim = U4(contents, &mut offset);
        let cmd_line = Cn(contents, &mut offset);

        Self {
            mod_tim,
            cmd_line,
        }
    }
}

impl fmt::Display for ATR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ATR:{}|{}", self.mod_tim, self.cmd_line)
    }
}

impl ATR {
    pub fn bytes(&self) -> Vec<u8> {
        let mut rec_len: i16 = 4;
        let rec_typ_sub: &[u8] = &[0u8, 20u8];

        let cmd_line_bytes: Vec<u8> = CnToBytes(self.cmd_line.clone(), &mut rec_len);
        [
            &rec_len.to_ne_bytes(),
            rec_typ_sub,
            &self.mod_tim.to_ne_bytes(),
            &cmd_line_bytes,
        ].concat()
    }
}

/// Master Information Record
#[derive(Debug, IntoPyObject, Clone)]
#[allow(dead_code)]
pub struct MIR {
    pub setup_t: u32,
    pub start_t: u32,
    pub stat_num: u8,
    pub mode_cod: char,
    pub rtst_cod: char,
    pub prot_cod: char,
    pub burn_tim: u16,
    pub cmod_cod: char,
    pub lot_id: String,
    pub part_typ: String,
    pub node_nam: String,
    pub tstr_typ: String,
    pub job_nam: String,
    pub job_rev: String,
    pub sblot_id: String,
    pub oper_nam: String,
    pub exec_typ: String,
    pub exec_ver: String,
    pub test_cod: String,
    pub tst_temp: String,
    pub user_txt: String,
    pub aux_file: String,
    pub pkg_typ: String,
    pub famly_id: String,
    pub date_cod: String,
    pub facil_id: String,
    pub floor_id: String,
    pub proc_id: String,
    pub oper_frq: String,
    pub spec_nam: String,
    pub spec_ver: String,
    pub flow_id: String,
    pub setup_id: String,
    pub dsgn_rev: String,
    pub eng_id: String,
    pub rom_cod: String,
    pub serl_num: String,
    pub supr_nam: String,
}

impl From<&RawRecord> for MIR {
    fn from(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let mut offset: usize = 0;
        let setup_t = U4(contents, &mut offset);
        let start_t = U4(contents, &mut offset);
        let stat_num = U1(contents, &mut offset);
        let mode_cod = C1(contents, &mut offset);
        let rtst_cod = C1(contents, &mut offset);
        let prot_cod = C1(contents, &mut offset);
        let burn_tim = U2(contents, &mut offset);
        let cmod_cod = C1(contents, &mut offset);
        let lot_id = Cn(contents, &mut offset);
        let part_typ = Cn(contents, &mut offset);
        let node_nam = Cn(contents, &mut offset);
        let tstr_typ = Cn(contents, &mut offset);
        let job_nam = Cn(contents, &mut offset);
        let job_rev = Cn(contents, &mut offset);
        let sblot_id = Cn(contents, &mut offset);
        let oper_nam = Cn(contents, &mut offset);
        let exec_typ = Cn(contents, &mut offset);
        let exec_ver = Cn(contents, &mut offset);
        let test_cod = Cn(contents, &mut offset);
        let tst_temp = Cn(contents, &mut offset);
        let user_txt = Cn(contents, &mut offset);
        let aux_file = Cn(contents, &mut offset);
        let pkg_typ = Cn(contents, &mut offset);
        let famly_id = Cn(contents, &mut offset);
        let date_cod = Cn(contents, &mut offset);
        let facil_id = Cn(contents, &mut offset);
        let floor_id = Cn(contents, &mut offset);
        let proc_id = Cn(contents, &mut offset);
        let oper_frq = Cn(contents, &mut offset);
        let spec_nam = Cn(contents, &mut offset);
        let spec_ver = Cn(contents, &mut offset);
        let flow_id = Cn(contents, &mut offset);
        let setup_id = Cn(contents, &mut offset);
        let dsgn_rev = Cn(contents, &mut offset);
        let eng_id = Cn(contents, &mut offset);
        let rom_cod = Cn(contents, &mut offset);
        let serl_num = Cn(contents, &mut offset);
        let supr_nam = Cn(contents, &mut offset);

        Self {
            setup_t,
            start_t,
            stat_num,
            mode_cod,
            rtst_cod,
            prot_cod,
            burn_tim,
            cmod_cod,
            lot_id,
            part_typ,
            node_nam,
            tstr_typ,
            job_nam,
            job_rev,
            sblot_id,
            oper_nam,
            exec_typ,
            exec_ver,
            test_cod,
            tst_temp,
            user_txt,
            aux_file,
            pkg_typ,
            famly_id,
            date_cod,
            facil_id,
            floor_id,
            proc_id,
            oper_frq,
            spec_nam,
            spec_ver,
            flow_id,
            setup_id,
            dsgn_rev,
            eng_id,
            rom_cod,
            serl_num,
            supr_nam,
        }
    }
}

impl fmt::Display for MIR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = Vec::new();
        result.push(self.lot_id.to_string());
        result.push(self.part_typ.to_string());
        result.push(self.job_nam.to_string());
        result.push(self.node_nam.to_string());
        result.push(self.tstr_typ.to_string());
        result.push(self.setup_t.to_string());
        result.push(self.start_t.to_string());
        result.push(self.oper_nam.to_string());
        result.push(self.mode_cod.to_string());
        result.push(self.stat_num.to_string());
        result.push(self.sblot_id.to_string());
        result.push(self.test_cod.to_string());
        result.push(self.rtst_cod.to_string());
        result.push(self.job_rev.to_string());
        result.push(self.exec_typ.to_string());
        result.push(self.exec_ver.to_string());
        result.push(self.prot_cod.to_string());
        result.push(self.cmod_cod.to_string());
        result.push(self.burn_tim.to_string());
        result.push(self.tst_temp.to_string());
        result.push(self.user_txt.to_string());
        result.push(self.aux_file.to_string());
        result.push(self.pkg_typ.to_string());
        result.push(self.famly_id.to_string());
        result.push(self.date_cod.to_string());
        result.push(self.facil_id.to_string());
        result.push(self.floor_id.to_string());
        result.push(self.proc_id.to_string());
        result.push(self.oper_frq.to_string());
        result.push(self.spec_nam.to_string());
        result.push(self.spec_ver.to_string());
        result.push(self.flow_id.to_string());
        result.push(self.setup_id.to_string());
        result.push(self.dsgn_rev.to_string());
        result.push(self.eng_id.to_string());
        result.push(self.rom_cod.to_string());
        result.push(self.serl_num.to_string());
        result.push(self.supr_nam.to_string());
        write!(f, "MIR:{}", result.join("|"))
    }
}

impl MIR {
    /// Get the MIR from a file at `fname`
    ///
    /// Opens the file from scratch and grabs the MIR rather than re-use some open file handle.
    /// The MIR is always right near the beginning of the file, so this is not very expensive
    ///
    /// # Error
    /// If for some reason the file cannot be parsed, returns an `std::io::Error`
    ///
    /// If for some reason an MIR cannot be found in the file, returns a
    /// `std::io::ErrorKind::UnexpectedEof`
    pub fn from_fname(fname: &str) -> std::io::Result<Self> {
        let records = Records::new(&fname)?;

        for record in records {
            if let Some(resolved) = record.resolve() {
                if let Record::MIR(mir) = resolved {
                    return Ok(mir);
                }
            }
        }
        Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "Failed to find MIR in file",
        ))
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut rec_len: i16 = 15;
        let rec_typ_sub: &[u8] = &[1u8, 10u8];

        let lot_id_bytes = CnToBytes(self.lot_id.clone(), &mut rec_len);
        let part_typ_bytes = CnToBytes(self.part_typ.clone(), &mut rec_len);
        let node_nam_bytes = CnToBytes(self.node_nam.clone(), &mut rec_len);
        let tstr_typ_bytes = CnToBytes(self.tstr_typ.clone(), &mut rec_len);
        let job_nam_bytes = CnToBytes(self.job_nam.clone(), &mut rec_len);
        let job_rev_bytes = CnToBytes(self.job_rev.clone(), &mut rec_len);
        let sblot_id_bytes = CnToBytes(self.sblot_id.clone(), &mut rec_len);
        let oper_nam_bytes = CnToBytes(self.oper_nam.clone(), &mut rec_len);
        let exec_typ_bytes = CnToBytes(self.exec_typ.clone(), &mut rec_len);
        let exec_ver_bytes = CnToBytes(self.exec_ver.clone(), &mut rec_len);
        let test_cod_bytes = CnToBytes(self.test_cod.clone(), &mut rec_len);
        let tst_temp_bytes = CnToBytes(self.tst_temp.clone(), &mut rec_len);
        let user_txt_bytes = CnToBytes(self.user_txt.clone(), &mut rec_len);
        let aux_file_bytes = CnToBytes(self.aux_file.clone(), &mut rec_len);
        let pkg_typ_bytes = CnToBytes(self.pkg_typ.clone(), &mut rec_len);
        let famly_id_bytes = CnToBytes(self.famly_id.clone(), &mut rec_len);
        let date_cod_bytes = CnToBytes(self.date_cod.clone(), &mut rec_len);
        let facil_id_bytes = CnToBytes(self.facil_id.clone(), &mut rec_len);
        let floor_id_bytes = CnToBytes(self.floor_id.clone(), &mut rec_len);
        let proc_id_bytes = CnToBytes(self.proc_id.clone(), &mut rec_len);
        let oper_frq_bytes = CnToBytes(self.oper_frq.clone(), &mut rec_len);
        let spec_nam_bytes = CnToBytes(self.spec_nam.clone(), &mut rec_len);
        let spec_ver_bytes = CnToBytes(self.spec_ver.clone(), &mut rec_len);
        let flow_id_bytes = CnToBytes(self.flow_id.clone(), &mut rec_len);
        let setup_id_bytes = CnToBytes(self.setup_id.clone(), &mut rec_len);
        let dsgn_rev_bytes = CnToBytes(self.dsgn_rev.clone(), &mut rec_len);
        let eng_id_bytes = CnToBytes(self.eng_id.clone(), &mut rec_len);
        let rom_cod_bytes = CnToBytes(self.rom_cod.clone(), &mut rec_len);
        let serl_num_bytes = CnToBytes(self.serl_num.clone(), &mut rec_len);
        let supr_nam_bytes = CnToBytes(self.supr_nam.clone(), &mut rec_len);

        [
            &rec_len.to_ne_bytes(),
            rec_typ_sub,
            &self.setup_t.to_ne_bytes(),
            &self.start_t.to_ne_bytes(),
            &self.stat_num.to_ne_bytes(),
            &[self.mode_cod as u8],
            &[self.rtst_cod as u8],
            &[self.prot_cod as u8],
            &self.burn_tim.to_ne_bytes(),
            &[self.cmod_cod as u8],
            &lot_id_bytes,
            &part_typ_bytes,
            &node_nam_bytes,
            &tstr_typ_bytes,
            &job_nam_bytes,
            &job_rev_bytes,
            &sblot_id_bytes,
            &oper_nam_bytes,
            &exec_typ_bytes,
            &exec_ver_bytes,
            &test_cod_bytes,
            &tst_temp_bytes,
            &user_txt_bytes,
            &aux_file_bytes,
            &pkg_typ_bytes,
            &famly_id_bytes,
            &date_cod_bytes,
            &facil_id_bytes,
            &floor_id_bytes,
            &proc_id_bytes,
            &oper_frq_bytes,
            &spec_nam_bytes,
            &spec_ver_bytes,
            &flow_id_bytes,
            &setup_id_bytes,
            &dsgn_rev_bytes,
            &eng_id_bytes,
            &rom_cod_bytes,
            &serl_num_bytes,
            &supr_nam_bytes,
        ].concat()
}
}

/// Site Description Record
#[derive(Debug, Clone, IntoPyObject)]
#[allow(dead_code)]
pub struct SDR {
    pub head_num: u8,
    pub site_grp: u8,
    pub site_cnt: u8,
    pub site_num: Vec<u8>,
    pub hand_typ: String,
    pub hand_id: String,
    pub card_typ: String,
    pub card_id: String,
    pub load_typ: String,
    pub load_id: String,
    pub dib_typ: String,
    pub dib_id: String,
    pub cabl_typ: String,
    pub cabl_id: String,
    pub cont_typ: String,
    pub cont_id: String,
    pub lasr_typ: String,
    pub lasr_id: String,
    pub extr_typ: String,
    pub extr_i: String,
}

impl From<&RawRecord> for SDR {
    fn from(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let mut offset: usize = 0;
        let head_num = U1(contents, &mut offset);
        let site_grp = U1(contents, &mut offset);
        let site_cnt = U1(contents, &mut offset);
        let site_num = kxU1(contents, site_cnt as usize, &mut offset);
        let hand_typ = Cn(contents, &mut offset);
        let hand_id = Cn(contents, &mut offset);
        let card_typ = Cn(contents, &mut offset);
        let card_id = Cn(contents, &mut offset);
        let load_typ = Cn(contents, &mut offset);
        let load_id = Cn(contents, &mut offset);
        let dib_typ = Cn(contents, &mut offset);
        let dib_id = Cn(contents, &mut offset);
        let cabl_typ = Cn(contents, &mut offset);
        let cabl_id = Cn(contents, &mut offset);
        let cont_typ = Cn(contents, &mut offset);
        let cont_id = Cn(contents, &mut offset);
        let lasr_typ = Cn(contents, &mut offset);
        let lasr_id = Cn(contents, &mut offset);
        let extr_typ = Cn(contents, &mut offset);
        let extr_i = Cn(contents, &mut offset);

        Self {
            head_num,
            site_grp,
            site_cnt,
            site_num,
            hand_typ,
            hand_id,
            card_typ,
            card_id,
            load_typ,
            load_id,
            dib_typ,
            dib_id,
            cabl_typ,
            cabl_id,
            cont_typ,
            cont_id,
            lasr_typ,
            lasr_id,
            extr_typ,
            extr_i,
        }
    }
}

impl fmt::Display for SDR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let site_num_str = self.site_num.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        let mut result = Vec::new();
        result.push(self.head_num.to_string());
        result.push(self.site_grp.to_string());
        result.push(self.site_cnt.to_string());
        result.push(site_num_str);
        result.push(self.hand_typ.to_string());
        result.push(self.hand_id.to_string());
        result.push(self.card_typ.to_string());
        result.push(self.card_id.to_string());
        result.push(self.load_typ.to_string());
        result.push(self.load_id.to_string());
        result.push(self.dib_typ.to_string());
        result.push(self.dib_id.to_string());
        result.push(self.cabl_typ.to_string());
        result.push(self.cabl_id.to_string());
        result.push(self.cont_typ.to_string());
        result.push(self.cont_id.to_string());
        result.push(self.lasr_typ.to_string());
        result.push(self.lasr_id.to_string());
        result.push(self.extr_typ.to_string());
        result.push(self.extr_i.to_string());
        write!(f, "SDR:{}", result.join("|"))
    }
}

impl SDR {
    pub fn bytes(&self) -> Vec<u8> {
        let mut rec_len: i16 = 3 + self.site_cnt as i16;
        let rec_typ_sub: &[u8] = &[1u8, 80u8];
        let hand_typ_bytes = CnToBytes(self.hand_typ.clone(), &mut rec_len);
        let hand_id_bytes = CnToBytes(self.hand_id.clone(), &mut rec_len);
        let card_typ_bytes = CnToBytes(self.card_typ.clone(), &mut rec_len);
        let card_id_bytes = CnToBytes(self.card_id.clone(), &mut rec_len);
        let load_typ_bytes = CnToBytes(self.load_typ.clone(), &mut rec_len);
        let load_id_bytes = CnToBytes(self.load_id.clone(), &mut rec_len);
        let dib_typ_bytes = CnToBytes(self.dib_typ.clone(), &mut rec_len);
        let dib_id_bytes = CnToBytes(self.dib_id.clone(), &mut rec_len);
        let cabl_typ_bytes = CnToBytes(self.cabl_typ.clone(), &mut rec_len);
        let cabl_id_bytes = CnToBytes(self.cabl_id.clone(), &mut rec_len);
        let cont_typ_bytes = CnToBytes(self.cont_typ.clone(), &mut rec_len);
        let cont_id_bytes = CnToBytes(self.cont_id.clone(), &mut rec_len);
        let lasr_typ_bytes = CnToBytes(self.lasr_typ.clone(), &mut rec_len);
        let lasr_id_bytes = CnToBytes(self.lasr_id.clone(), &mut rec_len);
        let extr_typ_bytes = CnToBytes(self.extr_typ.clone(), &mut rec_len);
        let extr_i_bytes = CnToBytes(self.extr_i.clone(), &mut rec_len);
        [
            &rec_len.to_ne_bytes(),
            rec_typ_sub,
            &self.head_num.to_ne_bytes(),
            &self.site_grp.to_ne_bytes(),
            &self.site_cnt.to_ne_bytes(),
            &self.site_num,
            &hand_typ_bytes,
            &hand_id_bytes,
            &card_typ_bytes,
            &card_id_bytes,
            &load_typ_bytes,
            &load_id_bytes,
            &dib_typ_bytes,
            &dib_id_bytes,
            &cabl_typ_bytes,
            &cabl_id_bytes,
            &cont_typ_bytes,
            &cont_id_bytes,
            &lasr_typ_bytes,
            &lasr_id_bytes,
            &extr_typ_bytes,
            &extr_i_bytes,
        ].concat()
    }
}

/// Test Synopsis Record
#[derive(Debug, IntoPyObject, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct TSR {
    pub head_num: u8,
    pub site_num: u8,
    pub test_typ: char,
    pub test_num: u32,
    pub exec_cnt: u32,
    pub fail_cnt: u32,
    pub alrm_cnt: u32,
    pub test_nam: String,
    pub seq_name: String,
    pub test_lbl: String,
    pub opt_flag: u8,
    pub test_tim: f32,
    pub test_min: f32,
    pub test_max: f32,
    pub tst_sums: f32,
    pub tst_sqrs: f32,
}

impl From<&RawRecord> for TSR {
    fn from(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let mut offset: usize = 0;
        let head_num = U1(contents, &mut offset);
        let site_num = U1(contents, &mut offset);
        let test_typ: char = C1(contents, &mut offset);
        let test_num = U4(contents, &mut offset);
        let exec_cnt = U4(contents, &mut offset);
        let fail_cnt = U4(contents, &mut offset);
        let alrm_cnt = U4(contents, &mut offset);
        let test_nam = Cn(contents, &mut offset);
        let seq_name = Cn(contents, &mut offset);
        let test_lbl = Cn(contents, &mut offset);
        let opt_flag = U1(contents, &mut offset);
        let test_tim = R4(contents, &mut offset);
        let test_min = R4(contents, &mut offset);
        let test_max = R4(contents, &mut offset);
        let tst_sums = R4(contents, &mut offset);
        let tst_sqrs = R4(contents, &mut offset);

        Self {
            head_num,
            site_num,
            test_typ,
            test_num,
            exec_cnt,
            fail_cnt,
            alrm_cnt,
            test_nam,
            seq_name,
            test_lbl,
            opt_flag,
            test_tim,
            test_min,
            test_max,
            tst_sums,
            tst_sqrs,
        }
    }
}

impl fmt::Display for TSR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = Vec::new();
        result.push(self.head_num.to_string());
        result.push(self.site_num.to_string());
        result.push(self.test_num.to_string());
        result.push(self.test_nam.to_string());
        result.push(self.test_typ.to_string());
        result.push(self.exec_cnt.to_string());
        result.push(self.fail_cnt.to_string());
        result.push(self.alrm_cnt.to_string());
        result.push(self.seq_name.to_string());
        result.push(self.test_lbl.to_string());
        result.push(self.test_tim.to_string());
        result.push(self.test_min.to_string());
        result.push(self.test_max.to_string());
        result.push(self.tst_sums.to_string());
        result.push(self.tst_sqrs.to_string());
        write!(f, "TSR:{}", result.join("|"))
    }
}

impl TSR {
    pub fn bytes(&self) -> Vec<u8> {
        let mut rec_len: i16 = 40;
        let rec_typ_sub: &[u8] = &[10u8, 30u8];
        let test_nam_bytes = CnToBytes(self.test_nam.clone(), &mut rec_len);
        let seq_name_bytes = CnToBytes(self.seq_name.clone(), &mut rec_len);
        let test_lbl_bytes = CnToBytes(self.test_lbl.clone(), &mut rec_len);
        [
            &rec_len.to_ne_bytes(),
            rec_typ_sub,
            &self.head_num.to_ne_bytes(),
            &self.site_num.to_ne_bytes(),
            &[self.test_typ as u8],
            &self.test_num.to_ne_bytes(),
            &self.exec_cnt.to_ne_bytes(),
            &self.fail_cnt.to_ne_bytes(),
            &self.alrm_cnt.to_ne_bytes(),
            &test_nam_bytes,
            &seq_name_bytes,
            &test_lbl_bytes,
            &self.opt_flag.to_ne_bytes(),
            &self.test_tim.to_ne_bytes(),
            &self.test_min.to_ne_bytes(),
            &self.test_max.to_ne_bytes(),
            &self.tst_sums.to_ne_bytes(),
            &self.tst_sqrs.to_ne_bytes(),
        ].concat()
    }
}

/// Software Bin Record
#[derive(Debug, Clone, IntoPyObject)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct SBR {
    pub head_num: u8,
    pub site_num: u8,
    pub sbin_num: u16,
    pub sbin_cnt: u32,
    pub sbin_pf: char,
    pub sbin_nam: String,
}

impl From<&RawRecord> for SBR {
    fn from(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let mut offset: usize = 0;
        let head_num = U1(contents, &mut offset);
        let site_num = U1(contents, &mut offset);
        let sbin_num = U2(contents, &mut offset);
        let sbin_cnt = U4(contents, &mut offset);
        let sbin_pf = C1(contents, &mut offset);
        let sbin_nam = Cn(contents, &mut offset);

        Self {
            head_num,
            site_num,
            sbin_num,
            sbin_cnt,
            sbin_pf,
            sbin_nam,
        }
    }
}

impl fmt::Display for SBR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = Vec::new();
        result.push(self.head_num.to_string());
        result.push(self.site_num.to_string());
        result.push(self.sbin_num.to_string());
        result.push(self.sbin_cnt.to_string());
        result.push(self.sbin_pf.to_string());
        result.push(self.sbin_nam.to_string());
        write!(f, "SBR:{}", result.join("|"))
    }
}

impl SBR {
    pub fn bytes(&self) -> Vec<u8> {
        let mut rec_len: i16 = 9;
        let rec_typ_sub: &[u8] = &[1u8, 50u8];
        let sbin_nam_bytes = CnToBytes(self.sbin_nam.clone(), &mut rec_len);
        [
            &rec_len.to_ne_bytes(),
            rec_typ_sub,
            &self.head_num.to_ne_bytes(),
            &self.site_num.to_ne_bytes(),
            &self.sbin_num.to_ne_bytes(),
            &self.sbin_cnt.to_ne_bytes(),
            &[self.sbin_pf as u8],
            &sbin_nam_bytes,
        ].concat()
    }
}

/// Wafer Information Record
#[derive(Debug, Clone, IntoPyObject)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct WIR {
    pub head_num: u8,
    pub site_grp: u8,
    pub start_t: u32,
    pub wafer_id: String,
}

impl From<&RawRecord> for WIR {
    fn from(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let mut offset: usize = 0;
        let head_num = U1(contents, &mut offset);
        let site_grp = U1(contents, &mut offset);
        let start_t = U4(contents, &mut offset);
        let wafer_id = Cn(contents, &mut offset);

        Self {
            head_num,
            site_grp,
            start_t,
            wafer_id,
        }
    }
}

impl fmt::Display for WIR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = Vec::new();
        result.push(self.head_num.to_string());
        result.push(self.start_t.to_string());
        result.push(self.site_grp.to_string());
        result.push(self.wafer_id.to_string());
        write!(f, "WIR:{}", result.join("|"))
    }
}

impl WIR {
    pub fn bytes(&self) -> Vec<u8> {
        let mut rec_len: i16 = 6;
        let rec_typ_sub: &[u8] = &[2u8, 10u8];
        let wafer_id_bytes = CnToBytes(self.wafer_id.clone(), &mut rec_len);
        [
            &rec_len.to_ne_bytes(),
            rec_typ_sub,
            &self.head_num.to_ne_bytes(),
            &self.site_grp.to_ne_bytes(),
            &self.start_t.to_ne_bytes(),
            &wafer_id_bytes,
        ].concat()
    }
}

/// Wafer Results Record
#[derive(Debug, IntoPyObject, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct WRR {
    pub head_num: u8,
    pub site_grp: u8,
    pub finish_t: u32,
    pub part_cnt: u32,
    pub rtst_cnt: u32,
    pub abrt_cnt: u32,
    pub good_cnt: u32,
    pub func_cnt: u32,
    pub wafer_id: String,
    pub fabwf_id: String,
    pub frame_id: String,
    pub mask_id: String,
    pub usr_desc: String,
    pub exc_desc: String,
}

impl From<&RawRecord> for WRR {
    fn from(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let mut offset: usize = 0;
        let head_num = U1(contents, &mut offset);
        let site_grp = U1(contents, &mut offset);
        let finish_t = U4(contents, &mut offset);
        let part_cnt = U4(contents, &mut offset);
        let rtst_cnt = U4(contents, &mut offset);
        let abrt_cnt = U4(contents, &mut offset);
        let good_cnt = U4(contents, &mut offset);
        let func_cnt = U4(contents, &mut offset);
        let wafer_id = Cn(contents, &mut offset);
        let fabwf_id = Cn(contents, &mut offset);
        let frame_id = Cn(contents, &mut offset);
        let mask_id = Cn(contents, &mut offset);
        let usr_desc = Cn(contents, &mut offset);
        let exc_desc = Cn(contents, &mut offset);

        Self {
            head_num,
            site_grp,
            finish_t,
            part_cnt,
            rtst_cnt,
            abrt_cnt,
            good_cnt,
            func_cnt,
            wafer_id,
            fabwf_id,
            frame_id,
            mask_id,
            usr_desc,
            exc_desc,
        }
    }
}

impl fmt::Display for WRR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = Vec::new();
        result.push(self.head_num.to_string());
        result.push(self.finish_t.to_string());
        result.push(self.part_cnt.to_string());
        result.push(self.wafer_id.to_string());
        result.push(self.site_grp.to_string());
        result.push(self.rtst_cnt.to_string());
        result.push(self.abrt_cnt.to_string());
        result.push(self.good_cnt.to_string());
        result.push(self.func_cnt.to_string());
        result.push(self.fabwf_id.to_string());
        result.push(self.frame_id.to_string());
        result.push(self.mask_id.to_string());
        result.push(self.usr_desc.to_string());
        result.push(self.exc_desc.to_string());
        write!(f, "WRR:{}", result.join("|"))
    }
}

impl WRR {
    pub fn bytes(&self) -> Vec<u8> {
        let mut rec_len: i16 = 26;
        let rec_typ_sub: &[u8] = &[2u8, 20u8];
        let wafer_id_bytes = CnToBytes(self.wafer_id.clone(), &mut rec_len);
        let fabwf_id_bytes = CnToBytes(self.fabwf_id.clone(), &mut rec_len);
        let frame_id_bytes = CnToBytes(self.frame_id.clone(), &mut rec_len);
        let mask_id_bytes = CnToBytes(self.mask_id.clone(), &mut rec_len);
        let usr_desc_bytes = CnToBytes(self.usr_desc.clone(), &mut rec_len);
        let exc_desc_bytes = CnToBytes(self.exc_desc.clone(), &mut rec_len);
        [
            &rec_len.to_ne_bytes(),
            rec_typ_sub,
            &self.head_num.to_ne_bytes(),
            &self.site_grp.to_ne_bytes(),
            &self.finish_t.to_ne_bytes(),
            &self.part_cnt.to_ne_bytes(),
            &self.rtst_cnt.to_ne_bytes(),
            &self.abrt_cnt.to_ne_bytes(),
            &self.good_cnt.to_ne_bytes(),
            &self.func_cnt.to_ne_bytes(),
            &wafer_id_bytes,
            &fabwf_id_bytes,
            &frame_id_bytes,
            &mask_id_bytes,
            &usr_desc_bytes,
            &exc_desc_bytes,
        ].concat()
    }
}

/// Hardware Bin Record
#[derive(Debug, Clone, IntoPyObject)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct HBR {
    pub head_num: u8,
    pub site_num: u8,
    pub hbin_num: u16,
    pub hbin_cnt: u32,
    pub hbin_pf: char,
    pub hbin_nam: String,
}

impl From<&RawRecord> for HBR {
    fn from(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let mut offset: usize = 0;
        let head_num = U1(contents, &mut offset);
        let site_num = U1(contents, &mut offset);
        let hbin_num = U2(contents, &mut offset);
        let hbin_cnt = U4(contents, &mut offset);
        let hbin_pf = C1(contents, &mut offset);
        let hbin_nam = Cn(contents, &mut offset);

        Self {
            head_num,
            site_num,
            hbin_num,
            hbin_cnt,
            hbin_pf,
            hbin_nam,
        }
    }
}

impl fmt::Display for HBR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = Vec::new();
        result.push(self.head_num.to_string());
        result.push(self.site_num.to_string());
        result.push(self.hbin_num.to_string());
        result.push(self.hbin_cnt.to_string());
        result.push(self.hbin_pf.to_string());
        result.push(self.hbin_nam.to_string());
        write!(f, "HBR:{}", result.join("|"))
    }
}

impl HBR {
    pub fn bytes(&self) -> Vec<u8> {
        let mut rec_len: i16 = 9;
        let rec_typ_sub: &[u8] = &[1u8, 40u8];
        let hbin_nam_bytes = CnToBytes(self.hbin_nam.clone(), &mut rec_len);
        [
            &rec_len.to_ne_bytes(),
            rec_typ_sub,
            &self.head_num.to_ne_bytes(),
            &self.site_num.to_ne_bytes(),
            &self.hbin_num.to_ne_bytes(),
            &self.hbin_cnt.to_ne_bytes(),
            &[self.hbin_pf as u8],
            &hbin_nam_bytes,
        ].concat()
    }
}

/// Part Count Record
#[derive(Debug, IntoPyObject, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct PCR {
    pub head_num: u8,
    pub site_num: u8,
    pub part_cnt: u32,
    pub rtst_cnt: u32,
    pub abrt_cnt: u32,
    pub good_cnt: u32,
    pub func_cnt: u32,
}

impl From<&RawRecord> for PCR {
    fn from(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let mut offset: usize = 0;
        let head_num = U1(contents, &mut offset);
        let site_num = U1(contents, &mut offset);
        let part_cnt = U4(contents, &mut offset);
        let rtst_cnt = U4(contents, &mut offset);
        let abrt_cnt = U4(contents, &mut offset);
        let good_cnt = U4(contents, &mut offset);
        let func_cnt = U4(contents, &mut offset);

        Self {
            head_num,
            site_num,
            part_cnt,
            rtst_cnt,
            abrt_cnt,
            good_cnt,
            func_cnt,
        }
    }
}

impl fmt::Display for PCR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = Vec::new();
        result.push(self.head_num.to_string());
        result.push(self.site_num.to_string());
        result.push(self.part_cnt.to_string());
        result.push(self.rtst_cnt.to_string());
        result.push(self.abrt_cnt.to_string());
        result.push(self.good_cnt.to_string());
        result.push(self.func_cnt.to_string());
        write!(f, "PCR:{}", result.join("|"))
    }
}

impl PCR {
    pub fn bytes(&self) -> Vec<u8> {
        let rec_len: i16 = 22;
        let rec_typ_sub: &[u8] = &[1u8, 30u8];
        [
            &rec_len.to_ne_bytes(),
            rec_typ_sub,
            &self.head_num.to_ne_bytes(),
            &self.site_num.to_ne_bytes(),
            &self.part_cnt.to_ne_bytes(),
            &self.rtst_cnt.to_ne_bytes(),
            &self.abrt_cnt.to_ne_bytes(),
            &self.good_cnt.to_ne_bytes(),
            &self.func_cnt.to_ne_bytes(),
        ].concat()
    }
}

/// Part Information Record
#[derive(Debug, IntoPyObject, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct PIR {
    pub head_num: u8,
    pub site_num: u8,
}

impl From<&RawRecord> for PIR {
    fn from(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let head_num = contents[0];
        let site_num = contents[1];

        Self { head_num, site_num }
    }
}

impl fmt::Display for PIR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PIR:{}|{}", self.head_num, self.site_num)
    }
}

impl PIR {
    pub fn bytes(&self) -> Vec<u8> {
        let rec_len: i16 = 2;
        let rec_typ_sub: &[u8] = &[5u8, 10u8];
        [
            &rec_len.to_ne_bytes(),
            rec_typ_sub,
            &self.head_num.to_ne_bytes(),
            &self.site_num.to_ne_bytes(),
        ].concat()
    }
}

/// Part Results Record
#[derive(Debug, IntoPyObject, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct PRR {
    pub head_num: u8,
    pub site_num: u8,
    pub part_flg: u8,
    pub num_test: u16,
    pub hard_bin: u16,
    pub soft_bin: u16,
    pub x_coord: i16,
    pub y_coord: i16,
    pub test_t: u32,
    pub part_id: String,
    pub part_txt: String,
    pub part_fix: Vec<u8>,
}

impl From<&RawRecord> for PRR {
    fn from(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let mut offset: usize = 0;
        let head_num = U1(contents, &mut offset);
        let site_num = U1(contents, &mut offset);
        let part_flg = U1(contents, &mut offset);
        let num_test = U2(contents, &mut offset);
        let hard_bin = U2(contents, &mut offset);
        let soft_bin = U2(contents, &mut offset);
        let x_coord = I2(contents, &mut offset);
        let y_coord = I2(contents, &mut offset);
        let test_t = U4(contents, &mut offset);
        let part_id = Cn(contents, &mut offset);
        let part_txt = Cn(contents, &mut offset);
        let part_fix = Bn(contents, &mut offset);

        Self {
            head_num,
            site_num,
            part_flg,
            num_test,
            hard_bin,
            soft_bin,
            x_coord,
            y_coord,
            test_t,
            part_id,
            part_txt,
            part_fix,
        }
    }
}

impl fmt::Display for PRR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let part_fix_str = self.part_fix.iter().map(|x| format!("{:02X}", x)).collect::<Vec<_>>().join("");
        let mut result = Vec::new();
        result.push(self.head_num.to_string());
        result.push(self.site_num.to_string());
        result.push(self.part_id.to_string());
        result.push(self.num_test.to_string());
        result.push(format!("{:02X}", self.part_flg));
        result.push(self.hard_bin.to_string());
        result.push(self.soft_bin.to_string());
        result.push(self.x_coord.to_string());
        result.push(self.y_coord.to_string());
        result.push(self.test_t.to_string());
        result.push(self.part_txt.to_string());
        result.push(part_fix_str);
        write!(f, "PRR:{}", result.join("|"))
    }
}

impl PRR {
    pub fn bytes(&self) -> Vec<u8> {
        let mut rec_len: i16 = 17;
        let rec_typ_sub: &[u8] = &[5u8, 20u8];
        let part_id_bytes = CnToBytes(self.part_id.clone(), &mut rec_len);
        let part_txt_bytes = CnToBytes(self.part_txt.clone(), &mut rec_len);
        rec_len += 1 + self.part_fix.len() as i16;
        let mut part_fix_bytes = vec![self.part_fix.len() as u8];
        part_fix_bytes.extend_from_slice(&self.part_fix);
        [
            &rec_len.to_ne_bytes(),
            rec_typ_sub,
            &self.head_num.to_ne_bytes(),
            &self.site_num.to_ne_bytes(),
            &self.part_flg.to_ne_bytes(),
            &self.num_test.to_ne_bytes(),
            &self.hard_bin.to_ne_bytes(),
            &self.soft_bin.to_ne_bytes(),
            &self.x_coord.to_ne_bytes(),
            &self.y_coord.to_ne_bytes(),
            &self.test_t.to_ne_bytes(),
            &part_id_bytes,
            &part_txt_bytes,
            &part_fix_bytes,
        ].concat()
    }
}

/// Master Results Record
#[derive(Debug, IntoPyObject, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct MRR {
    pub finish_t: u32,
    pub disp_cod: char,
    pub usr_desc: String,
    pub exc_desc: String,
}

impl From<&RawRecord> for MRR {
    fn from(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let mut offset: usize = 0;
        let finish_t = U4(contents, &mut offset);
        let disp_cod = C1(contents, &mut offset);
        let usr_desc = Cn(contents, &mut offset);
        let exc_desc = Cn(contents, &mut offset);

        Self {
            finish_t,
            disp_cod,
            usr_desc,
            exc_desc,
        }
    }
}

impl fmt::Display for MRR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = Vec::new();
        result.push(self.finish_t.to_string());
        result.push(self.disp_cod.to_string());
        result.push(self.usr_desc.to_string());
        result.push(self.exc_desc.to_string());
        write!(f, "MRR:{}", result.join("|"))
    }
}

impl MRR {
    pub fn bytes(&self) -> Vec<u8> {
        let mut rec_len: i16 = 5;
        let rec_typ_sub: &[u8] = &[1u8, 20u8];
        let usr_desc_bytes = CnToBytes(self.usr_desc.clone(), &mut rec_len);
        let exc_desc_bytes = CnToBytes(self.exc_desc.clone(), &mut rec_len);
        [
            &rec_len.to_ne_bytes(),
            rec_typ_sub,
            &self.finish_t.to_ne_bytes(),
            &[self.disp_cod as u8],
            &usr_desc_bytes,
            &exc_desc_bytes,
        ].concat()
    }
}

/// Parametric Test Record
#[derive(Debug, IntoPyObject, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct PTR {
    pub test_num: u32,
    pub head_num: u8,
    pub site_num: u8,
    pub test_flg: u8,
    pub parm_flg: u8,
    pub result: f32,
    pub test_txt: String,
    pub alarm_id: String,
    pub opt_flag: u8,
    pub res_scal: i8,
    pub llm_scal: i8,
    pub hlm_scal: i8,
    pub lo_limit: f32,
    pub hi_limit: f32,
    pub units: String,
    pub c_resfmt: String,
    pub c_llmfmt: String,
    pub c_hlmfmt: String,
    pub lo_spec: f32,
    pub hi_spec: f32,
}

impl From<&RawRecord> for PTR {
    fn from(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let mut offset: usize = 0;
        let test_num = U4(contents, &mut offset);
        let head_num = U1(contents, &mut offset);
        let site_num = U1(contents, &mut offset);
        let test_flg = U1(contents, &mut offset);
        let parm_flg = U1(contents, &mut offset);
        let result = R4(contents, &mut offset);
        let test_txt = Cn(contents, &mut offset);
        let alarm_id = Cn(contents, &mut offset);
        let opt_flag;
        let res_scal;
        let llm_scal;
        let hlm_scal;
        let lo_limit;
        let hi_limit;
        let units;
        let c_resfmt;
        let c_llmfmt;
        let c_hlmfmt;
        let lo_spec;
        let hi_spec;
        if offset < record.contents.len() {
            opt_flag = U1(contents, &mut offset);
            res_scal = I1(contents, &mut offset);
            llm_scal = I1(contents, &mut offset);
            hlm_scal = I1(contents, &mut offset);
            lo_limit = R4(contents, &mut offset);
            hi_limit = R4(contents, &mut offset);
            units = Cn(contents, &mut offset);
            c_resfmt = Cn(contents, &mut offset);
            c_llmfmt = Cn(contents, &mut offset);
            c_hlmfmt = Cn(contents, &mut offset);
            lo_spec = R4(contents, &mut offset);
            hi_spec = R4(contents, &mut offset);
        } else {
            opt_flag = 0;
            res_scal = 0;
            llm_scal = 0;
            hlm_scal = 0;
            lo_limit = 0.;
            hi_limit = 0.;
            units = "".to_string();
            c_resfmt = "".to_string();
            c_llmfmt = "".to_string();
            c_hlmfmt = "".to_string();
            lo_spec = 0.;
            hi_spec = 0.;
        }

        Self {
            test_num,
            head_num,
            site_num,
            test_flg,
            parm_flg,
            result,
            test_txt,
            alarm_id,
            opt_flag,
            res_scal,
            llm_scal,
            hlm_scal,
            lo_limit,
            hi_limit,
            units,
            c_resfmt,
            c_llmfmt,
            c_hlmfmt,
            lo_spec,
            hi_spec,
        }
    }
}

impl PTR {
    pub fn pass(&self) -> bool {
        (self.test_flg >> 6) & 0b11 == 0
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut rec_len: i16 = 32;
        let rec_typ_sub: &[u8] = &[15u8, 10u8];
        let test_txt_bytes = CnToBytes(self.test_txt.clone(), &mut rec_len);
        let alarm_id_bytes = CnToBytes(self.alarm_id.clone(), &mut rec_len);
        let units_bytes = CnToBytes(self.units.clone(), &mut rec_len);
        let c_resfmt_bytes = CnToBytes(self.c_resfmt.clone(), &mut rec_len);
        let c_llmfmt_bytes = CnToBytes(self.c_llmfmt.clone(), &mut rec_len);
        let c_hlmfmt_bytes = CnToBytes(self.c_hlmfmt.clone(), &mut rec_len);
        [
            &rec_len.to_ne_bytes(),
            rec_typ_sub,
            &self.test_num.to_ne_bytes(),
            &self.head_num.to_ne_bytes(),
            &self.site_num.to_ne_bytes(),
            &self.test_flg.to_ne_bytes(),
            &self.parm_flg.to_ne_bytes(),
            &self.result.to_ne_bytes(),
            &test_txt_bytes,
            &alarm_id_bytes,
            &self.opt_flag.to_ne_bytes(),
            &self.res_scal.to_ne_bytes(),
            &self.llm_scal.to_ne_bytes(),
            &self.hlm_scal.to_ne_bytes(),
            &self.lo_limit.to_ne_bytes(),
            &self.hi_limit.to_ne_bytes(),
            &units_bytes,
            &c_resfmt_bytes,
            &c_llmfmt_bytes,
            &c_hlmfmt_bytes,
            &self.lo_spec.to_ne_bytes(),
            &self.hi_spec.to_ne_bytes(),
        ].concat()
    }
}

impl fmt::Display for PTR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = Vec::new();
        result.push(self.test_num.to_string());
        result.push(self.head_num.to_string());
        result.push(self.site_num.to_string());
        result.push(self.result.to_string());
        result.push(format!("{:02X}", self.test_flg));
        result.push(format!("{:02X}", self.parm_flg));
        result.push(self.test_txt.to_string());
        result.push(self.alarm_id.to_string());
        result.push(self.units.to_string());
        result.push(self.lo_limit.to_string());
        result.push(self.hi_limit.to_string());
        result.push(self.c_resfmt.to_string());
        result.push(self.c_llmfmt.to_string());
        result.push(self.c_hlmfmt.to_string());
        result.push(self.lo_spec.to_string());
        result.push(self.hi_spec.to_string());
        result.push(self.res_scal.to_string());
        result.push(self.llm_scal.to_string());
        result.push(self.hlm_scal.to_string());
        write!(f, "PTR:{}", result.join("|"))
    }
}

/// Functional Test Record
#[derive(Debug, IntoPyObject, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct FTR {
    pub test_num: u32,
    pub head_num: u8,
    pub site_num: u8,
    pub test_flg: u8,
    pub opt_flag: u8,
    pub cycl_cnt: u32,
    pub rel_vadr: u32,
    pub rept_cnt: u32,
    pub num_fail: u32,
    pub xfail_ad: i32,
    pub yfail_ad: i32,
    pub vect_off: i16,
    pub rtn_icnt: u16,      // j
    pub pgm_icnt: u16,      // k
    pub rtn_indx: Vec<u16>, // rtn_icnt
    pub rtn_stat: Vec<u8>,  // rtn_icnt, nibbles
    pub pgm_indx: Vec<u16>, // pgm_icnt
    pub pgm_stat: Vec<u8>,  // pgm_icnt, nibbles
    pub fail_pin: Vec<u8>,  // Dn type (first 2 bytes length)
    pub vect_nam: String,
    pub time_set: String,
    pub op_code: String,
    pub test_txt: String,
    pub alarm_id: String,
    pub prog_txt: String,
    pub rslt_txt: String,
    pub patg_num: u8,
    pub spin_map: Vec<u8>, // Dn type (first 2 bytes length)
}

impl From<&RawRecord> for FTR {
    fn from(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let mut offset: usize = 0;
        let test_num = U4(contents, &mut offset);
        let head_num = U1(contents, &mut offset);
        let site_num = U1(contents, &mut offset);
        let test_flg = U1(contents, &mut offset);
        let opt_flag = U1(contents, &mut offset);
        let cycl_cnt = U4(contents, &mut offset);
        let rel_vadr = U4(contents, &mut offset);
        let rept_cnt = U4(contents, &mut offset);
        let num_fail = U4(contents, &mut offset);
        let xfail_ad = I4(contents, &mut offset);
        let yfail_ad = I4(contents, &mut offset);
        let vect_off = I2(contents, &mut offset);
        let rtn_icnt = U2(contents, &mut offset);
        let pgm_icnt = U2(contents, &mut offset);
        let rtn_indx = kxU2(contents, rtn_icnt.into(), &mut offset);
        let rtn_stat = kxN1(contents, rtn_icnt.into(), &mut offset);
        let pgm_indx = kxU2(contents, pgm_icnt.into(), &mut offset);
        let pgm_stat = kxN1(contents, pgm_icnt.into(), &mut offset);
        let fail_pin = Dn(contents, &mut offset);

        let vect_nam = Cn(contents, &mut offset);
        let time_set = Cn(contents, &mut offset);
        let op_code = Cn(contents, &mut offset);
        let test_txt = Cn(contents, &mut offset);
        let alarm_id = Cn(contents, &mut offset);
        let prog_txt = Cn(contents, &mut offset);
        let rslt_txt = Cn(contents, &mut offset);
        let patg_num = U1(contents, &mut offset);
        let spin_map = Dn(contents, &mut offset);

        Self {
            test_num,
            head_num,
            site_num,
            test_flg,
            opt_flag,
            cycl_cnt,
            rel_vadr,
            rept_cnt,
            num_fail,
            xfail_ad,
            yfail_ad,
            vect_off,
            rtn_icnt,
            pgm_icnt,
            rtn_indx,
            rtn_stat,
            pgm_indx,
            pgm_stat,
            fail_pin,
            vect_nam,
            time_set,
            op_code,
            test_txt,
            alarm_id,
            prog_txt,
            rslt_txt,
            patg_num,
            spin_map,
        }
    }
}

impl FTR {
    pub fn get_passfail(&self) -> bool {
        let test_flg = self.test_flg;
        // don't bother checking the other flags
        test_flg & 0x80 == 0
    }

    pub fn bytes(&self) -> Vec<u8> {
        let rtn_stat_nbytes = (self.rtn_icnt as usize).div_ceil(2) as i16;
        let pgm_stat_nbytes = (self.pgm_icnt as usize).div_ceil(2) as i16;
        let mut rec_len: i16 = 38
            + self.rtn_icnt as i16 * 2 + rtn_stat_nbytes
            + self.pgm_icnt as i16 * 2 + pgm_stat_nbytes
            + 2 + self.fail_pin.len() as i16
            + 1
            + 2 + self.spin_map.len() as i16;
        let rec_typ_sub: &[u8] = &[15u8, 20u8];

        let rtn_indx_bytes: Vec<u8> = self.rtn_indx.iter().flat_map(|x| x.to_ne_bytes()).collect();
        let rtn_stat_bytes: Vec<u8> = (0..rtn_stat_nbytes as usize).map(|i| {
            let lo = if i * 2 < self.rtn_stat.len() { self.rtn_stat[i * 2] & 0xf } else { 0 };
            let hi = if i * 2 + 1 < self.rtn_stat.len() { self.rtn_stat[i * 2 + 1] & 0xf } else { 0 };
            lo | (hi << 4)
        }).collect();
        let pgm_indx_bytes: Vec<u8> = self.pgm_indx.iter().flat_map(|x| x.to_ne_bytes()).collect();
        let pgm_stat_bytes: Vec<u8> = (0..pgm_stat_nbytes as usize).map(|i| {
            let lo = if i * 2 < self.pgm_stat.len() { self.pgm_stat[i * 2] & 0xf } else { 0 };
            let hi = if i * 2 + 1 < self.pgm_stat.len() { self.pgm_stat[i * 2 + 1] & 0xf } else { 0 };
            lo | (hi << 4)
        }).collect();
        let fail_pin_nbits = (self.fail_pin.len() * 8) as u16;
        let mut fail_pin_bytes = fail_pin_nbits.to_ne_bytes().to_vec();
        fail_pin_bytes.extend_from_slice(&self.fail_pin);
        let spin_map_nbits = (self.spin_map.len() * 8) as u16;
        let mut spin_map_bytes = spin_map_nbits.to_ne_bytes().to_vec();
        spin_map_bytes.extend_from_slice(&self.spin_map);

        let vect_nam_bytes = CnToBytes(self.vect_nam.clone(), &mut rec_len);
        let time_set_bytes = CnToBytes(self.time_set.clone(), &mut rec_len);
        let op_code_bytes = CnToBytes(self.op_code.clone(), &mut rec_len);
        let test_txt_bytes = CnToBytes(self.test_txt.clone(), &mut rec_len);
        let alarm_id_bytes = CnToBytes(self.alarm_id.clone(), &mut rec_len);
        let prog_txt_bytes = CnToBytes(self.prog_txt.clone(), &mut rec_len);
        let rslt_txt_bytes = CnToBytes(self.rslt_txt.clone(), &mut rec_len);

        [
            &rec_len.to_ne_bytes(),
            rec_typ_sub,
            &self.test_num.to_ne_bytes(),
            &self.head_num.to_ne_bytes(),
            &self.site_num.to_ne_bytes(),
            &self.test_flg.to_ne_bytes(),
            &self.opt_flag.to_ne_bytes(),
            &self.cycl_cnt.to_ne_bytes(),
            &self.rel_vadr.to_ne_bytes(),
            &self.rept_cnt.to_ne_bytes(),
            &self.num_fail.to_ne_bytes(),
            &self.xfail_ad.to_ne_bytes(),
            &self.yfail_ad.to_ne_bytes(),
            &self.vect_off.to_ne_bytes(),
            &self.rtn_icnt.to_ne_bytes(),
            &self.pgm_icnt.to_ne_bytes(),
            &rtn_indx_bytes,
            &rtn_stat_bytes,
            &pgm_indx_bytes,
            &pgm_stat_bytes,
            &fail_pin_bytes,
            &vect_nam_bytes,
            &time_set_bytes,
            &op_code_bytes,
            &test_txt_bytes,
            &alarm_id_bytes,
            &prog_txt_bytes,
            &rslt_txt_bytes,
            &self.patg_num.to_ne_bytes(),
            &spin_map_bytes,
        ].concat()
    }
}

impl fmt::Display for FTR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rtn_indx_str = self.rtn_indx.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        let rtn_stat_str = self.rtn_stat.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        let pgm_indx_str = self.pgm_indx.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        let pgm_stat_str = self.pgm_stat.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        let fail_pin_str = self.fail_pin.iter().map(|x| format!("{:02X}", x)).collect::<Vec<_>>().join("");
        let spin_map_str = self.spin_map.iter().map(|x| format!("{:02X}", x)).collect::<Vec<_>>().join("");
        let mut result = Vec::new();
        result.push(self.test_num.to_string());
        result.push(self.head_num.to_string());
        result.push(self.site_num.to_string());
        result.push(format!("{:02X}", self.test_flg));
        result.push(self.vect_nam.to_string());
        result.push(self.time_set.to_string());
        result.push(self.cycl_cnt.to_string());
        result.push(self.rel_vadr.to_string());
        result.push(self.rept_cnt.to_string());
        result.push(self.num_fail.to_string());
        result.push(self.xfail_ad.to_string());
        result.push(self.yfail_ad.to_string());
        result.push(self.vect_off.to_string());
        result.push(rtn_indx_str);
        result.push(rtn_stat_str);
        result.push(pgm_indx_str);
        result.push(pgm_stat_str);
        result.push(fail_pin_str);
        result.push(self.op_code.to_string());
        result.push(self.test_txt.to_string());
        result.push(self.alarm_id.to_string());
        result.push(self.prog_txt.to_string());
        result.push(self.rslt_txt.to_string());
        result.push(self.patg_num.to_string());
        result.push(spin_map_str);
        write!(f, "FTR:{}", result.join("|"))
    }
}

/// Multiple-Result Parametric Record
#[derive(Debug, IntoPyObject, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct MPR {
    pub test_num: u32,
    pub head_num: u8,
    pub site_num: u8,
    pub test_flg: u8,
    pub parm_flg: u8,
    pub rtn_icnt: u16,      // j
    pub rslt_cnt: u16,      // k
    pub rtn_stat: Vec<u8>,  // jxN*1
    pub rtn_rslt: Vec<f32>, // kxR*4
    pub test_txt: String,
    pub alarm_id: String,
    pub opt_flag: u8,
    pub res_scal: i8,
    pub llm_scal: i8,
    pub hlm_scal: i8,
    pub lo_limit: f32,
    pub hi_limit: f32,
    pub start_in: f32,
    pub incr_in: f32,
    pub rtn_indx: Vec<u16>, // jxU*2
    pub units: String,
    pub units_in: String,
    pub c_resfmt: String,
    pub c_llmfmt: String,
    pub c_hlmfmt: String,
    pub lo_spec: f32,
    pub hi_spec: f32,
}

impl From<&RawRecord> for MPR {
    fn from(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let mut offset: usize = 0;

        let test_num = U4(contents, &mut offset);
        let head_num = U1(contents, &mut offset);
        let site_num = U1(contents, &mut offset);
        let test_flg = U1(contents, &mut offset);
        let parm_flg = U1(contents, &mut offset);
        let rtn_icnt = U2(contents, &mut offset);
        let rslt_cnt = U2(contents, &mut offset);
        let rtn_stat = kxN1(contents, rtn_icnt.into(), &mut offset);
        let rtn_rslt = kxR4(contents, rslt_cnt.into(), &mut offset);
        let test_txt = Cn(contents, &mut offset);
        let alarm_id = Cn(contents, &mut offset);
        let opt_flag = U1(contents, &mut offset);
        let res_scal = I1(contents, &mut offset);
        let llm_scal = I1(contents, &mut offset);
        let hlm_scal = I1(contents, &mut offset);
        let lo_limit = R4(contents, &mut offset);
        let hi_limit = R4(contents, &mut offset);
        let start_in = R4(contents, &mut offset);
        let incr_in = R4(contents, &mut offset);
        let rtn_indx = kxU2(contents, rtn_icnt.into(), &mut offset);
        let units = Cn(contents, &mut offset);
        let units_in = Cn(contents, &mut offset);
        let c_resfmt = Cn(contents, &mut offset);
        let c_llmfmt = Cn(contents, &mut offset);
        let c_hlmfmt = Cn(contents, &mut offset);
        let lo_spec = R4(contents, &mut offset);
        let hi_spec = R4(contents, &mut offset);

        Self {
            test_num,
            head_num,
            site_num,
            test_flg,
            parm_flg,
            rtn_icnt,
            rslt_cnt,
            rtn_stat,
            rtn_rslt,
            test_txt,
            alarm_id,
            opt_flag,
            res_scal,
            llm_scal,
            hlm_scal,
            lo_limit,
            hi_limit,
            start_in,
            incr_in,
            rtn_indx,
            units,
            units_in,
            c_resfmt,
            c_llmfmt,
            c_hlmfmt,
            lo_spec,
            hi_spec,
        }
    }
}

impl fmt::Display for MPR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rtn_stat_str = self.rtn_stat.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        let rtn_rslt_str = self.rtn_rslt.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        let rtn_indx_str = self.rtn_indx.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        let mut result = Vec::new();
        result.push(self.test_num.to_string());
        result.push(self.head_num.to_string());
        result.push(self.site_num.to_string());
        result.push(rtn_stat_str);
        result.push(rtn_rslt_str);
        result.push(format!("{:02X}", self.test_flg));
        result.push(self.test_txt.to_string());
        result.push(self.alarm_id.to_string());
        result.push(format!("{:02X}", self.parm_flg));
        result.push(self.units.to_string());
        result.push(self.lo_limit.to_string());
        result.push(self.hi_limit.to_string());
        result.push(self.start_in.to_string());
        result.push(self.incr_in.to_string());
        result.push(self.units_in.to_string());
        result.push(rtn_indx_str);
        result.push(self.c_resfmt.to_string());
        result.push(self.c_llmfmt.to_string());
        result.push(self.c_hlmfmt.to_string());
        result.push(self.lo_spec.to_string());
        result.push(self.hi_spec.to_string());
        result.push(self.res_scal.to_string());
        result.push(self.llm_scal.to_string());
        result.push(self.hlm_scal.to_string());
        write!(f, "MPR:{}", result.join("|"))
    }
}

impl MPR {
    pub fn bytes(&self) -> Vec<u8> {
        let rtn_stat_nbytes = (self.rtn_icnt as usize).div_ceil(2) as i16;
        let mut rec_len: i16 = 40 + rtn_stat_nbytes + self.rslt_cnt as i16 * 4 + self.rtn_icnt as i16 * 2;
        let rec_typ_sub: &[u8] = &[15u8, 15u8];
        let rtn_stat_bytes: Vec<u8> = (0..rtn_stat_nbytes as usize).map(|i| {
            let lo = if i * 2 < self.rtn_stat.len() { self.rtn_stat[i * 2] & 0xf } else { 0 };
            let hi = if i * 2 + 1 < self.rtn_stat.len() { self.rtn_stat[i * 2 + 1] & 0xf } else { 0 };
            lo | (hi << 4)
        }).collect();
        let rtn_rslt_bytes: Vec<u8> = self.rtn_rslt.iter().flat_map(|x| x.to_ne_bytes()).collect();
        let test_txt_bytes = CnToBytes(self.test_txt.clone(), &mut rec_len);
        let alarm_id_bytes = CnToBytes(self.alarm_id.clone(), &mut rec_len);
        let rtn_indx_bytes: Vec<u8> = self.rtn_indx.iter().flat_map(|x| x.to_ne_bytes()).collect();
        let units_bytes = CnToBytes(self.units.clone(), &mut rec_len);
        let units_in_bytes = CnToBytes(self.units_in.clone(), &mut rec_len);
        let c_resfmt_bytes = CnToBytes(self.c_resfmt.clone(), &mut rec_len);
        let c_llmfmt_bytes = CnToBytes(self.c_llmfmt.clone(), &mut rec_len);
        let c_hlmfmt_bytes = CnToBytes(self.c_hlmfmt.clone(), &mut rec_len);
        [
            &rec_len.to_ne_bytes(),
            rec_typ_sub,
            &self.test_num.to_ne_bytes(),
            &self.head_num.to_ne_bytes(),
            &self.site_num.to_ne_bytes(),
            &self.test_flg.to_ne_bytes(),
            &self.parm_flg.to_ne_bytes(),
            &self.rtn_icnt.to_ne_bytes(),
            &self.rslt_cnt.to_ne_bytes(),
            &rtn_stat_bytes,
            &rtn_rslt_bytes,
            &test_txt_bytes,
            &alarm_id_bytes,
            &self.opt_flag.to_ne_bytes(),
            &self.res_scal.to_ne_bytes(),
            &self.llm_scal.to_ne_bytes(),
            &self.hlm_scal.to_ne_bytes(),
            &self.lo_limit.to_ne_bytes(),
            &self.hi_limit.to_ne_bytes(),
            &self.start_in.to_ne_bytes(),
            &self.incr_in.to_ne_bytes(),
            &rtn_indx_bytes,
            &units_bytes,
            &units_in_bytes,
            &c_resfmt_bytes,
            &c_llmfmt_bytes,
            &c_hlmfmt_bytes,
            &self.lo_spec.to_ne_bytes(),
            &self.hi_spec.to_ne_bytes(),
        ].concat()
    }
}

/// Pin Map Record
#[derive(Debug, IntoPyObject, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct PMR {
    pub pmr_indx: u16,
    pub chan_typ: u16,
    pub chan_nam: String,
    pub phy_nam: String,
    pub log_nam: String,
    pub head_num: u8,
    pub site_num: u8,
}

impl From<&RawRecord> for PMR {
    fn from(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let mut offset: usize = 0;

        let pmr_indx = U2(contents, &mut offset);
        let chan_typ = U2(contents, &mut offset);
        let chan_nam = Cn(contents, &mut offset);
        let phy_nam = Cn(contents, &mut offset);
        let log_nam = Cn(contents, &mut offset);
        let head_num = U1(contents, &mut offset);
        let site_num = U1(contents, &mut offset);

        Self {
            pmr_indx,
            chan_typ,
            chan_nam,
            phy_nam,
            log_nam,
            head_num,
            site_num,
        }
    }
}

impl fmt::Display for PMR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = Vec::new();
        result.push(self.pmr_indx.to_string());
        result.push(self.chan_typ.to_string());
        result.push(self.chan_nam.to_string());
        result.push(self.phy_nam.to_string());
        result.push(self.log_nam.to_string());
        result.push(self.head_num.to_string());
        result.push(self.site_num.to_string());
        write!(f, "PMR:{}", result.join("|"))
    }
}

impl PMR {
    pub fn bytes(&self) -> Vec<u8> {
        let mut rec_len: i16 = 6;
        let rec_typ_sub: &[u8] = &[1u8, 60u8];
        let chan_nam_bytes = CnToBytes(self.chan_nam.clone(), &mut rec_len);
        let phy_nam_bytes = CnToBytes(self.phy_nam.clone(), &mut rec_len);
        let log_nam_bytes = CnToBytes(self.log_nam.clone(), &mut rec_len);
        [
            &rec_len.to_ne_bytes(),
            rec_typ_sub,
            &self.pmr_indx.to_ne_bytes(),
            &self.chan_typ.to_ne_bytes(),
            &chan_nam_bytes,
            &phy_nam_bytes,
            &log_nam_bytes,
            &self.head_num.to_ne_bytes(),
            &self.site_num.to_ne_bytes(),
        ].concat()
    }
}

/// Pin Group Record
#[derive(Debug, IntoPyObject, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct PGR {
    pub grp_indx: u16,
    pub grp_nam: String,
    pub indx_cnt: u16,
    pub pmr_indx: Vec<u16>,
}

impl From<&RawRecord> for PGR {
    fn from(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let mut offset: usize = 0;

        let grp_indx = U2(contents, &mut offset);
        let grp_nam = Cn(contents, &mut offset);
        let indx_cnt = U2(contents, &mut offset);
        let pmr_indx = kxU2(contents, indx_cnt.into(), &mut offset);

        Self {
            grp_indx,
            grp_nam,
            indx_cnt,
            pmr_indx
        }
    }
}

impl fmt::Display for PGR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pmr_indx_str = self.pmr_indx.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        let mut result = Vec::new();
        result.push(self.grp_indx.to_string());
        result.push(self.grp_nam.to_string());
        result.push(self.indx_cnt.to_string());
        result.push(pmr_indx_str);
        write!(f, "PGR:{}", result.join("|"))
    }
}

impl PGR {
    pub fn bytes(&self) -> Vec<u8> {
        let mut rec_len: i16 = 4 + self.indx_cnt as i16 * 2;
        let rec_typ_sub: &[u8] = &[1u8, 62u8];
        let grp_nam_bytes = CnToBytes(self.grp_nam.clone(), &mut rec_len);
        let pmr_indx_bytes: Vec<u8> = self.pmr_indx.iter().flat_map(|x| x.to_ne_bytes()).collect();
        [
            &rec_len.to_ne_bytes(),
            rec_typ_sub,
            &self.grp_indx.to_ne_bytes(),
            &grp_nam_bytes,
            &self.indx_cnt.to_ne_bytes(),
            &pmr_indx_bytes,
        ].concat()
    }
}

/// Pin List Record
#[derive(Debug, IntoPyObject, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct PLR {
    pub grp_cnt: u16,
    pub grp_indx: Vec<u16>,
    pub grp_mode: Vec<u16>,
    pub grp_radx: Vec<u8>,
    pub pgm_char: Vec<String>,
    pub rtn_char: Vec<String>,
    pub pgm_chal: Vec<String>,
    pub rtn_chal: Vec<String>
}

impl From<&RawRecord> for PLR {
    fn from(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let mut offset: usize = 0;

        let grp_cnt = U2(contents, &mut offset);
        let grp_indx = kxU2(contents, grp_cnt.into(), &mut offset);
        let grp_mode = kxU2(contents, grp_cnt.into(), &mut offset);
        let grp_radx = kxU1(contents, grp_cnt.into(), &mut offset);
        let pgm_char = kxCn(contents, grp_cnt.into(), &mut offset);
        let rtn_char = kxCn(contents, grp_cnt.into(), &mut offset);
        let pgm_chal = kxCn(contents, grp_cnt.into(), &mut offset);
        let rtn_chal = kxCn(contents, grp_cnt.into(), &mut offset);

        Self {
            grp_cnt,
            grp_indx,
            grp_mode,
            grp_radx,
            pgm_char,
            rtn_char,
            pgm_chal,
            rtn_chal,
        }
    }
}

impl fmt::Display for PLR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let grp_indx_str = self.grp_indx.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        let grp_mode_str = self.grp_mode.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        let grp_radx_str = self.grp_radx.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        let pgm_char_str = self.pgm_char.join(",");
        let rtn_char_str = self.rtn_char.join(",");
        let pgm_chal_str = self.pgm_chal.join(",");
        let rtn_chal_str = self.rtn_chal.join(",");
        let mut result = Vec::new();
        result.push(grp_indx_str);
        result.push(grp_mode_str);
        result.push(grp_radx_str);
        result.push(format!("{}, {}",pgm_chal_str, pgm_char_str));
        result.push(format!("{}, {}",rtn_chal_str, rtn_char_str));
        write!(f, "PLR:{}", result.join("|"))
    }
}

impl PLR {
    pub fn bytes(&self) -> Vec<u8> {
        let grp_cnt = self.grp_cnt as i16;
        let mut rec_len: i16 = 2 + grp_cnt * 5;
        let rec_typ_sub: &[u8] = &[1u8, 63u8];
        let grp_indx_bytes: Vec<u8> = self.grp_indx.iter().flat_map(|x| x.to_ne_bytes()).collect();
        let grp_mode_bytes: Vec<u8> = self.grp_mode.iter().flat_map(|x| x.to_ne_bytes()).collect();
        let pgm_char_bytes: Vec<u8> = self.pgm_char.iter().flat_map(|s| {
            let b = s.as_bytes(); let mut v = vec![b.len() as u8]; v.extend_from_slice(b); v
        }).collect();
        rec_len += pgm_char_bytes.len() as i16;
        let rtn_char_bytes: Vec<u8> = self.rtn_char.iter().flat_map(|s| {
            let b = s.as_bytes(); let mut v = vec![b.len() as u8]; v.extend_from_slice(b); v
        }).collect();
        rec_len += rtn_char_bytes.len() as i16;
        let pgm_chal_bytes: Vec<u8> = self.pgm_chal.iter().flat_map(|s| {
            let b = s.as_bytes(); let mut v = vec![b.len() as u8]; v.extend_from_slice(b); v
        }).collect();
        rec_len += pgm_chal_bytes.len() as i16;
        let rtn_chal_bytes: Vec<u8> = self.rtn_chal.iter().flat_map(|s| {
            let b = s.as_bytes(); let mut v = vec![b.len() as u8]; v.extend_from_slice(b); v
        }).collect();
        rec_len += rtn_chal_bytes.len() as i16;
        [
            &rec_len.to_ne_bytes(),
            rec_typ_sub,
            &self.grp_cnt.to_ne_bytes(),
            &grp_indx_bytes,
            &grp_mode_bytes,
            &self.grp_radx,
            &pgm_char_bytes,
            &rtn_char_bytes,
            &pgm_chal_bytes,
            &rtn_chal_bytes,
        ].concat()
    }
}

/// Retest Data Record
#[derive(Debug, IntoPyObject, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct RDR {
    pub num_bins: u16,
    pub rtst_bin: Vec<u16>,
}

impl From<&RawRecord> for RDR {
    fn from(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let mut offset: usize = 0;
        let num_bins = U2(contents, &mut offset);
        let rtst_bin = kxU2(contents, num_bins.into(), &mut offset);

        Self {
            num_bins,
            rtst_bin
        }
    }
}

impl fmt::Display for RDR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rtst_bin_str = self.rtst_bin.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        write!(f, "RDR:{}|{}", self.num_bins, rtst_bin_str)
    }
}

impl RDR {
    pub fn bytes(&self) -> Vec<u8> {
        let rec_len: i16 = 2 + self.num_bins as i16 * 2;
        let rec_typ_sub: &[u8] = &[1u8, 70u8];
        let rtst_bin_bytes: Vec<u8> = self.rtst_bin.iter().flat_map(|x| x.to_ne_bytes()).collect();
        [
            &rec_len.to_ne_bytes(),
            rec_typ_sub,
            &self.num_bins.to_ne_bytes(),
            &rtst_bin_bytes,
        ].concat()
    }
}

/// Wafer Configuration Record
#[derive(Debug, IntoPyObject, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct WCR {
    pub wafr_siz: f32,
    pub die_ht: f32,
    pub die_wid: f32,
    pub wf_units: u8,
    pub wf_flat: char,
    pub center_x: i16,
    pub center_y: i16,
    pub pos_x: char,
    pub pos_y: char,
}

impl From<&RawRecord> for WCR {
    fn from(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let mut offset: usize = 0;
        let wafr_siz = R4(contents, &mut offset);
        let die_ht = R4(contents, &mut offset);
        let die_wid = R4(contents, &mut offset);
        let wf_units = U1(contents, &mut offset);
        let wf_flat = C1(contents, &mut offset);
        let center_x = I2(contents, &mut offset);
        let center_y = I2(contents, &mut offset);
        let pos_x = C1(contents, &mut offset);
        let pos_y = C1(contents, &mut offset);

        Self {
            wafr_siz,
            die_ht,
            die_wid,
            wf_units,
            wf_flat,
            center_x,
            center_y,
            pos_x,
            pos_y,
        }
    }
}

impl fmt::Display for WCR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = Vec::new();
        result.push(self.wf_flat.to_string());
        result.push(self.pos_x.to_string());
        result.push(self.pos_y.to_string());
        result.push(self.wafr_siz.to_string());
        result.push(self.die_ht.to_string());
        result.push(self.die_wid.to_string());
        result.push(self.wf_units.to_string());
        result.push(self.center_x.to_string());
        result.push(self.center_y.to_string());
        write!(f, "WCR:{}", result.join("|"))
    }
}

impl WCR {
    pub fn bytes(&self) -> Vec<u8> {
        let rec_len: i16 = 20;
        let rec_typ_sub: &[u8] = &[2u8, 30u8];
        [
            &rec_len.to_ne_bytes(),
            rec_typ_sub,
            &self.wafr_siz.to_ne_bytes(),
            &self.die_ht.to_ne_bytes(),
            &self.die_wid.to_ne_bytes(),
            &self.wf_units.to_ne_bytes(),
            &[self.wf_flat as u8],
            &self.center_x.to_ne_bytes(),
            &self.center_y.to_ne_bytes(),
            &[self.pos_x as u8],
            &[self.pos_y as u8],
        ].concat()
    }
}

/// Begin Program Section Record
#[derive(Debug, IntoPyObject, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct BPS {
    pub  seq_name: String,
}

impl From<&RawRecord> for BPS {
    fn from(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let mut offset: usize = 0;
        let seq_name = Cn(contents, &mut offset);

        Self {
            seq_name,
        }
    }
}

impl fmt::Display for BPS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BPS:{}", self.seq_name)
    }
}

impl BPS {
    pub fn bytes(&self) -> Vec<u8> {
        let mut rec_len: i16 = 0;
        let rec_typ_sub: &[u8] = &[20u8, 10u8];
        let seq_name_bytes = CnToBytes(self.seq_name.clone(), &mut rec_len);
        [
            &rec_len.to_ne_bytes(),
            rec_typ_sub,
            &seq_name_bytes,
        ].concat()
    }
}

/// Begin Program Section Record
/// Does not contain any data
/// Location: Following the corresponding BPS and before PRR in the data stream.
/// Possible Use:
/// When performing analyses on a particular program segment’s test.
/// Note that pairs of BPS and EPS records can be nested: for example, when one sequencer
/// calls another. In this case, the sequence of records could look like this:
///     BPS SEQ_NAME = sequence-1
///     BPS SEQ_NAME = sequence-2
///     EPS (end of sequence-2)
///     EPS (end of sequence-1)
/// Because an EPS record does not contain the name of the sequencer, it should be
/// assumed that each EPS record matches the last unmatched BPS record.
#[derive(Debug, Clone, IntoPyObject)]
#[allow(dead_code)]
pub struct EPS {
    dummy_field: String,
}

impl From<&RawRecord> for EPS {
    fn from(_record: &RawRecord) -> Self {
        let dummy_field = "".to_string();
        Self {
            dummy_field,
        }
    }
}

impl fmt::Display for EPS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EPS:")
    }
}

impl EPS {
    pub fn bytes(&self) -> Vec<u8> {
        let rec_len: i16 = 0;
        let rec_typ_sub: &[u8] = &[20u8, 20u8];
        [
            &rec_len.to_ne_bytes(),
            rec_typ_sub,
        ].concat()
    }
}

/// Generic Data Record
#[derive(Debug, Clone, IntoPyObject)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct GDR {
    pub fld_cnt: u16,
    pub gen_data: Vec<GenData>,
}

impl From<&RawRecord> for GDR {
    fn from(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let mut offset: usize = 0;
        let fld_cnt = U2(contents, &mut offset);
        let gen_data = Vn(contents, fld_cnt.into(), &mut offset);

        Self {
            fld_cnt,
            gen_data,
        }
    }
}

impl fmt::Display for GDR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let gen_data_str = self.gen_data.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        write!(f, "GDR:{}|{}", self.fld_cnt, gen_data_str)
    }
}

impl GDR {
    pub fn bytes(&self) -> Vec<u8> {
        let rec_typ_sub: &[u8] = &[50u8, 10u8];
        let gen_data_bytes: Vec<u8> = self.gen_data.iter().flat_map(|d| {
            let mut b: Vec<u8> = Vec::new();
            match d {
                GenData::U1(v) => { b.push(1);  b.extend_from_slice(&v.to_ne_bytes()); }
                GenData::U2(v) => { b.push(2);  b.extend_from_slice(&v.to_ne_bytes()); }
                GenData::U4(v) => { b.push(3);  b.extend_from_slice(&v.to_ne_bytes()); }
                GenData::I1(v) => { b.push(4);  b.extend_from_slice(&v.to_ne_bytes()); }
                GenData::I2(v) => { b.push(5);  b.extend_from_slice(&v.to_ne_bytes()); }
                GenData::I4(v) => { b.push(6);  b.extend_from_slice(&v.to_ne_bytes()); }
                GenData::R4(v) => { b.push(7);  b.extend_from_slice(&v.to_ne_bytes()); }
                GenData::R8(v) => { b.push(8);  b.extend_from_slice(&v.to_ne_bytes()); }
                GenData::Cn(v) => {
                    b.push(10);
                    let s = v.as_bytes();
                    b.push(s.len() as u8);
                    b.extend_from_slice(s);
                }
                GenData::Bn(v) => {
                    b.push(11);
                    b.push(v.len() as u8);
                    b.extend_from_slice(v);
                }
                GenData::Dn(v) => {
                    b.push(12);
                    let nbits = (v.len() * 8) as u16;
                    b.extend_from_slice(&nbits.to_ne_bytes());
                    b.extend_from_slice(v);
                }
                GenData::N1(v) => { b.push(13); b.push(*v & 0xf); }
            }
            b
        }).collect();
        let rec_len: i16 = 2 + gen_data_bytes.len() as i16;
        [
            &rec_len.to_ne_bytes(),
            rec_typ_sub,
            &self.fld_cnt.to_ne_bytes(),
            &gen_data_bytes,
        ].concat()
    }
}

/// Datalog Text Record
#[derive(Debug, IntoPyObject, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct DTR {
    pub  text_dat: String,
}

impl From<&RawRecord> for DTR {
    fn from(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let mut offset: usize = 0;
        let text_dat = Cn(contents, &mut offset);

        Self {
            text_dat,
        }
    }
}

impl fmt::Display for DTR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DTR:{}", self.text_dat)
    }
}

impl DTR {
    pub fn bytes(&self) -> Vec<u8> {
        let mut rec_len: i16 = 0;
        let rec_typ_sub: &[u8] = &[50u8, 30u8];
        let text_dat_bytes = CnToBytes(self.text_dat.clone(), &mut rec_len);
        [
            &rec_len.to_ne_bytes(),
            rec_typ_sub,
            &text_dat_bytes,
        ].concat()
    }
}


#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct NotImplementedRecord {}

/// An enum of all the concrete record types
#[derive(Debug, Clone, IntoPyObject)]
pub enum Record {
    FAR(FAR),
    ATR(ATR),
    MIR(MIR),
    MRR(MRR),
    PCR(PCR),
    HBR(HBR),
    SBR(SBR),
    PMR(PMR),
    PGR(PGR),
    PLR(PLR),
    RDR(RDR),
    SDR(SDR),
    WIR(WIR),
    WRR(WRR),
    WCR(WCR),
    PIR(PIR),
    PRR(PRR),
    TSR(TSR),
    PTR(PTR),
    MPR(MPR),
    FTR(FTR),
    BPS(BPS),
    EPS(EPS),
    GDR(GDR),
    DTR(DTR),
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        macro_rules! display {
            ($($variant:ident),*) => {
                match self {
                    $(Record::$variant(r) => write!(f, "{}", r),)*
                }
            };
        }
        display!(FAR, ATR, MIR, MRR, PCR, HBR, SBR, PMR, PGR, PLR, RDR,
                 SDR, WIR, WRR, WCR, PIR, PRR, TSR, PTR, MPR, FTR, BPS, EPS, GDR, DTR)
    }
}

impl Record {
    pub fn bytes(&self) -> Vec<u8> {
        macro_rules! dispatch {
            ($($variant:ident),*) => {
                match self {
                    $(Record::$variant(r) => r.bytes(),)*
                }
            };
        }
        dispatch!(FAR, ATR, MIR, MRR, PCR, HBR, SBR, PMR, PGR, PLR, RDR,
                  SDR, WIR, WRR, WCR, PIR, PRR, TSR, PTR, MPR, FTR, BPS, EPS, GDR, DTR)
    }
}
