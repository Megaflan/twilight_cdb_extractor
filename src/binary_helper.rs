use std::{io::Read, u32};

// Unsigned types
pub fn read_byte<R: Read>(reader: &mut R) -> u8 {
    let mut buffer = [0u8; 1];
    reader.read_exact(&mut buffer).expect("Failed to read byte");
    buffer[0]
}

pub fn read_bytes<R: Read>(reader: &mut R, n: usize) -> Vec<u8> {
    let mut buffer = vec![0u8; n];
    reader.read_exact(&mut buffer).expect("Failed to read bytes");
    buffer
}

pub fn read_ushort<R: Read>(reader: &mut R) -> u16 {
    let mut buffer = [0u8; 2];
    reader.read_exact(&mut buffer).expect("Failed to read ushort");
    u16::from_le_bytes(buffer)
}

pub fn read_uint<R: Read>(reader: &mut R) -> u32 {
    let mut buffer = [0u8; 4];
    reader.read_exact(&mut buffer).expect("Failed to read uint");
    u32::from_le_bytes(buffer)
}

pub fn read_ulong<R: Read>(reader: &mut R) -> u64 {
    let mut buffer = [0u8; 8];
    reader.read_exact(&mut buffer).expect("Failed to read ulong");
    u64::from_le_bytes(buffer)
}

// Signed types
pub fn read_sbyte<R: Read>(reader: &mut R) -> i8 {
    let mut buffer = [0u8; 1];
    reader.read_exact(&mut buffer).expect("Failed to read sbyte");
    buffer[0] as i8
}

pub fn read_sbytes<R: Read>(reader: &mut R, n: usize) -> Vec<i8> {
    let mut buffer = vec![0u8; n];
    reader.read_exact(&mut buffer).expect("Failed to read bytes");
    buffer.into_iter().map(|byte| byte as i8).collect()
}

pub fn read_short<R: Read>(reader: &mut R) -> i16 {
    let mut buffer = [0u8; 2];
    reader.read_exact(&mut buffer).expect("Failed to read short");
    i16::from_le_bytes(buffer)
}

pub fn read_int<R: Read>(reader: &mut R) -> i32 {
    let mut buffer = [0u8; 4];
    reader.read_exact(&mut buffer).expect("Failed to read int");
    i32::from_le_bytes(buffer)
}

pub fn read_long<R: Read>(reader: &mut R) -> i64 {
    let mut buffer = [0u8; 8];
    reader.read_exact(&mut buffer).expect("Failed to read long");
    i64::from_le_bytes(buffer)
}

// Floating-point types
pub fn read_float<R: Read>(reader: &mut R) -> f32 {
    let mut buffer = [0u8; 4];
    reader.read_exact(&mut buffer).expect("Failed to read float");
    f32::from_le_bytes(buffer)
}

pub fn read_double<R: Read>(reader: &mut R) -> f64 {
    let mut buffer = [0u8; 8];
    reader.read_exact(&mut buffer).expect("Failed to read double");
    f64::from_le_bytes(buffer)
}
