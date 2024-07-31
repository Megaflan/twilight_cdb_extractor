use std::io;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct Entry {
    pub offset: u32,
    pub length: u32,
}

pub fn read_file(file_path: &std::path::Path) -> io::Result<Vec<u8>> {
    let mut stream: Vec<u8> = Vec::new();
    File::open(&file_path)?.read_to_end(&mut stream)?;
    Ok(stream)
}

pub fn process_entries(stream: Vec<u8>) -> Vec<Entry> {
    let mut entries: Vec<Entry> = Vec::new();
    let mut stream_pos: usize = 0;

    loop {
        let offset = u16::from_le_bytes([stream[stream_pos], stream[stream_pos + 1]]) as u32;
        let length = u16::from_le_bytes([stream[stream_pos + 2], stream[stream_pos + 3]]) as u32;

        if offset == 0 && length == 0 {
            break;
        }

        entries.push(Entry {
            offset: offset * 0x800,
            length: length * 0x800,
        });

        stream_pos = stream_pos + 4;
    }

    entries
}