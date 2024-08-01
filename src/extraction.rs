use std::io;
use std::fs::File;
use std::io::Read;

use crate::compression;

#[derive(Debug)]
pub struct Entry {
    pub offset: u32,
    pub length: u32,
    pub data: Vec<u8>
}

pub fn read_file(file_path: &std::path::Path) -> io::Result<Vec<u8>> {
    let mut stream: Vec<u8> = Vec::new();
    File::open(&file_path)?.read_to_end(&mut stream)?;
    Ok(stream)
}

pub fn process_entries(stream: Vec<u8>, compressed: bool) -> Vec<Entry> {
    let mut entries: Vec<Entry> = Vec::new();
    let mut stream_pos: usize = 0;

    loop {
        let offset = u16::from_le_bytes([stream[stream_pos], stream[stream_pos + 1]]) as u32 * 0x800;
        let length = u16::from_le_bytes([stream[stream_pos + 2], stream[stream_pos + 3]]) as u32 * 0x800;

        if offset == 0 && length == 0 {
            break;
        }

        let mut data = stream[offset as usize..(offset + length) as usize].to_vec();

        if compressed {
            data = compression::decompress(data);
        }

        entries.push(Entry {
            offset: offset,
            length: length,
            data
        });

        stream_pos = stream_pos + 4;
    }

    entries
}