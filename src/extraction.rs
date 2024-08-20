use std::io::{self, Seek, SeekFrom};
use std::fs::File;
use std::io::Read;

use crate::binary_helper;
use binary_helper::*;

#[derive(Debug)]
pub struct Entry {
    pub offset: u32,
    pub length: u32,
    pub format: String,
    pub data: Vec<u8>
}

pub fn read_file(file_path: &std::path::Path) -> Vec<Entry> {
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut entries: Vec<Entry> = Vec::new();

    loop {
        let offset = read_ushort(&mut file) as u32 * 0x800;
        let length = read_ushort(&mut file) as u32 * 0x800;

        if offset == 0 && length == 0 {
            break;
        }

        let pos = file.stream_position().expect("Failed to obtain stream position");

        file.seek(SeekFrom::Start(offset as u64)).expect("Failed to seek entry position");

        let data = read_bytes(&mut file, length as usize);

        if !data.is_empty() {
            entries.push(Entry {
                offset,
                length,
                format: identify_format(data[..4].to_vec()),
                data,
            });
        }       

        file.seek(SeekFrom::Start(pos)).expect("Failed to seek entry position");;
    }
    entries
}

fn identify_format(data: Vec<u8>) -> String {    
    if data.starts_with(&[0x70, 0x42, 0x41, 0x56]) {
        "vab".to_string() //VAB (Sony SDK Audio Format)
    } else if data.starts_with(&[0x42, 0x47, 0x50, 0x20]) {        
        "bgp".to_string() //BGP (Binary Pixel Data 4BPP)
    } else if data.starts_with(&[0x50, 0x53, 0x44, 0x42]) {
        "psdb".to_string() //PSDB (TIM Container)
    } else if data.starts_with(&[0x54, 0x4D, 0x53, 0x20]) {
        "tms".to_string() //TMS (TIM Container)
    } else {        
        "bin".to_string() //BIN (Binary Data)
    }
}