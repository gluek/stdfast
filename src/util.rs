//! utility functions for parsing STDF data types
#![allow(non_snake_case)]

use std::fmt;
use pyo3::IntoPyObject;

/// Parse a uint8 and advance the `offset`
pub fn U1(bytes: &[u8], offset: &mut usize) -> u8 {
    let x = bytes[*offset];
    *offset += 1;
    x
}

/// Parse a uint16 and advance the `offset`
pub fn U2(bytes: &[u8], offset: &mut usize) -> u16 {
    let x = u16::from_le_bytes(bytes[*offset..*offset + 2].try_into().unwrap());
    *offset += 2;
    x
}

/// Parse a uint32 and advance the `offset`
pub fn U4(bytes: &[u8], offset: &mut usize) -> u32 {
    let x = u32::from_le_bytes(bytes[*offset..*offset + 4].try_into().unwrap());
    *offset += 4;
    x
}

/// Parse a int8 and advance the `offset`
pub fn I1(bytes: &[u8], offset: &mut usize) -> i8 {
    let x = bytes[*offset] as i8;
    *offset += 1;
    x
}

/// Parse a int16 and advance the `offset`
pub fn I2(bytes: &[u8], offset: &mut usize) -> i16 {
    let x = i16::from_le_bytes(bytes[*offset..*offset + 2].try_into().unwrap());
    *offset += 2;
    x
}

/// Parse a int32 and advance the `offset`
pub fn I4(bytes: &[u8], offset: &mut usize) -> i32 {
    let x = i32::from_le_bytes(bytes[*offset..*offset + 4].try_into().unwrap());
    *offset += 4;
    x
}

/// Parse a 32-bit float and advance the `offset`
pub fn R4(bytes: &[u8], offset: &mut usize) -> f32 {
    let x = f32::from_le_bytes(bytes[*offset..*offset + 4].try_into().unwrap());
    *offset += 4;
    x
}

/// Parse a 64-bit float and advance the `offset`
pub fn R8(bytes: &[u8], offset: &mut usize) -> f64 {
    let x = f64::from_le_bytes(bytes[*offset..*offset + 8].try_into().unwrap());
    *offset += 8;
    x
}

/// Parse a single 8-bit character and advance the `offset`
pub fn C1(bytes: &[u8], offset: &mut usize) -> char {
    let x = char::from_u32(bytes[*offset] as u32)
        .expect("Failed to parse C1 from {offset} from\n{bytes:#?}");
    *offset += 1;
    x
}

/// Parse a string and advance the `offset`
pub fn Cn(bytes: &[u8], offset: &mut usize) -> String {
    let length = bytes[*offset] as usize;
    let result = String::from_utf8(bytes[*offset + 1..*offset + 1 + length].to_vec());
    if let Ok(s) = result {
        *offset += 1 + length;
        s
    } else {
        panic!("Failed to parse Cn from {offset} with length {length} from\n{bytes:#?}");
    }
}

/// Convert string to bytes and increase rec_len
pub fn CnToBytes(content: String, rec_len: &mut i16) -> Vec<u8>{
    let content_bytes = content.into_bytes();
    *rec_len += i16::try_from(content_bytes.len()).unwrap();
    content_bytes
}

/// Parse an array of bits and advance the `offset`
pub fn Bn(bytes: &[u8], offset: &mut usize) -> Vec<u8> {
    let length = bytes[*offset] as usize;
    let x = bytes[*offset + 1..*offset + 1 + length].to_vec();
    *offset += 1 + length;
    x
}

/// Parse an array of uint8 and advance the offset
pub fn Dn(bytes: &[u8], offset: &mut usize) -> Vec<u8> {
    let nbits = u16::from_le_bytes(bytes[*offset..*offset + 2].try_into().unwrap()) as usize;
    let length = nbits.div_ceil(8);
    let dn = bytes[*offset + 2..*offset + 2 + length].to_vec();
    *offset += 2 + length;
    dn
}

/// Parse an array of uint8 and advance the offset
pub fn kxU1(contents: &[u8], num: usize, offset: &mut usize) -> Vec<u8> {
    let x = contents[*offset..*offset + num].to_vec();
    *offset += num;
    x
}

/// Parse an array of uint16 and advance the offset
pub fn kxU2(contents: &[u8], num: usize, offset: &mut usize) -> Vec<u16> {
    let mut v = Vec::with_capacity(num as usize);
    for _ in 0..num {
        let x = u16::from_le_bytes(contents[*offset..*offset + 2].try_into().unwrap());
        v.push(x);
        *offset += 2;
    }
    v
}

/// Parse an array of f32 and advance the offset
pub fn kxR4(contents: &[u8], num: usize, offset: &mut usize) -> Vec<f32> {
    let mut v = Vec::with_capacity(num);
    for _ in 0..num {
        let x = f32::from_le_bytes(contents[*offset..*offset + 4].try_into().unwrap());
        v.push(x);
        *offset += 4;
    }
    v
}

/// Parse an array of uint4 and advance the offset
pub fn kxN1(contents: &[u8], num: usize, offset: &mut usize) -> Vec<u8> {
    let nbytes = num.div_ceil(2) as usize;
    let mut v = Vec::with_capacity(num as usize);
    for _ in 0..nbytes {
        let x = contents[*offset];
        v.push((x >> 0) & 0xf); // lower nibble
        v.push((x >> 4) & 0xf); // upper nibble
        *offset += 1;
    }
    v
}

/// Parse an array of string and advance the offset
pub fn kxCn(bytes: &[u8], num: usize, offset: &mut usize) -> Vec<String> {
    let mut v = Vec::with_capacity(num);
    for _ in 0..num {
        let length = bytes[*offset] as usize;
        let result = String::from_utf8(bytes[*offset + 1..*offset + 1 + length].to_vec());
        if let Ok(s) = result {
            *offset += 1 + length;
            v.push(s);
        } else {
            panic!("Failed to parse kxCn from {offset} with num {num} and length {length} from\n{bytes:#?}");
        }
    }
    v
}

#[derive(Debug, Clone, IntoPyObject)]
pub enum GenData {
    U1(u8),
    U2(u16),
    U4(u32),
    I1(i8),
    I2(i16),
    I4(i32),
    R4(f32),
    R8(f64),
    Cn(String),
    Bn(Vec<u8>),
    Dn(Vec<u8>),
    N1(u8)
}

impl fmt::Display for GenData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GenData::U1(v) => write!(f, "{}", v),
            GenData::U2(v) => write!(f, "{}", v),
            GenData::U4(v) => write!(f, "{}", v),
            GenData::I1(v) => write!(f, "{}", v),
            GenData::I2(v) => write!(f, "{}", v),
            GenData::I4(v) => write!(f, "{}", v),
            GenData::R4(v) => write!(f, "{}", v),
            GenData::R8(v) => write!(f, "{}", v),
            GenData::Cn(v) => write!(f, "{}", v),
            GenData::Bn(v) => write!(f, "{}", v.iter().map(|x| format!("{:02X}", x)).collect::<Vec<_>>().join("")),
            GenData::Dn(v) => write!(f, "{}", v.iter().map(|x| format!("{:02X}", x)).collect::<Vec<_>>().join("")),
            GenData::N1(v) => write!(f, "{}", v),
        }
    }
}

/// Parse an array of GenData and advance the offset
pub fn Vn(bytes: &[u8], num: usize, offset: &mut usize) -> Vec<GenData> {
    let mut v = Vec::with_capacity(num);
    for _ in 0..num {
        let dtype_code = bytes[*offset] as u8;
        *offset += 1;
        match dtype_code {
            0 => (),
            1 => v.push(GenData::U1(U1(bytes, offset))),
            2 => v.push(GenData::U2(U2(bytes, offset))),
            3 => v.push(GenData::U4(U4(bytes, offset))),
            4 => v.push(GenData::I1(I1(bytes, offset))),
            5 => v.push(GenData::I2(I2(bytes, offset))),
            6 => v.push(GenData::I4(I4(bytes, offset))),
            7 => v.push(GenData::R4(R4(bytes, offset))),
            8 => v.push(GenData::R8(R8(bytes, offset))),
            10 => v.push(GenData::Cn(Cn(bytes, offset))),
            11 => v.push(GenData::Bn(Bn(bytes, offset))),
            12 => v.push(GenData::Dn(Dn(bytes, offset))),
            13 => v.push(GenData::N1(U1(bytes, offset) & 0xf)),
            _ => (),
        }
    }
    v
}