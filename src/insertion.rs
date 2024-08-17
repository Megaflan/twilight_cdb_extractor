use std::io::{Seek, SeekFrom, Write};
use std::fs::{File, ReadDir};
use std::io::Read;

#[derive(Debug)]
pub struct Entry {
    pub offset: u32,
    pub length: u32,
    pub data: Vec<u8>
}

pub fn repack(entries: ReadDir, dir_path: &str) -> File {
    let mut stream = File::create(format!("{}.cdb", dir_path)).unwrap();
    let mut file_entries: Vec<Entry> = Vec::new();

    //Preparations
    for entry_result in entries {        
        let entry = entry_result.unwrap();  
        let file_name = entry.file_name().into_string().unwrap();
        let offset_str = file_name.split('_').nth(1).unwrap().replace(".bin", "");  
        let offset = offset_str.parse::<u32>().unwrap();  
        
        let mut data = Vec::new();
        let file_result = std::fs::File::open(entry.path());  
        if let Ok(mut file) = file_result {
            file.read_to_end(&mut data).unwrap();  
        
            let file_entry = Entry {
                offset,
                length: data.len() as u32,
                data,
            };
    
            file_entries.push(file_entry);
        }        
    }

    //Writing header into the file...
    for entry in &file_entries {
        let offset = (entry.offset / 0x800) as u16;
        let length = (entry.length / 0x800) as u16;

        let offset_bytes = offset.to_le_bytes();
        let length_bytes = length.to_le_bytes();
    
        stream.write_all(&offset_bytes).unwrap();
        stream.write_all(&length_bytes).unwrap();
    }
    //Writing data into the file...
    for entry in &file_entries {
        stream.seek(SeekFrom::Start(entry.offset as u64)).unwrap();
        stream.write_all(&entry.data).unwrap();
    }

    stream
}