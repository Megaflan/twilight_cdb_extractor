use std::fs::{File, ReadDir};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

#[derive(Debug)]
struct Entry {
    offset: u32,
    length: u32,
    data: Vec<u8>,
}

pub fn repack(entries: ReadDir, dir_path: &str) -> File {
    let mut stream =
        File::create(format!("{}.cdb", dir_path)).expect("Failed to create output file");
    let mut file_entries: Vec<Entry> = Vec::new();

    // Preparations
    for entry_result in entries {
        let entry = entry_result.expect("Failed to read directory entry");
        let mut file_name = entry
            .file_name()
            .into_string()
            .expect("Failed to convert file name to string");
        file_name.truncate(file_name.len() - 4);
        let offset_str = file_name
            .split('_')
            .nth(1)
            .expect("Invalid file name format");
        let offset = offset_str
            .parse::<u32>()
            .expect("Failed to parse offset from file name");

        let mut data = Vec::new();
        let mut file = File::open(entry.path()).expect("Failed to open input file");
        file.read_to_end(&mut data)
            .expect("Failed to read input file");

        let file_entry = Entry {
            offset,
            length: data.len() as u32,
            data,
        };

        file_entries.push(file_entry);
    }

    // Sort entries by offset
    file_entries.sort_by_key(|e| e.offset);

    // Adjust offsets if necessary
    let mut current_offset = 0;
    for entry in &mut file_entries {
        if entry.offset < current_offset {
            entry.offset = current_offset;
        }
        current_offset = entry.offset + entry.length;
        // Align to the next 0x800 boundary
        current_offset = (current_offset + 0x7FF) & !0x7FF;
    }

    // Writing header into the file...
    for entry in &file_entries {
        let offset = (entry.offset / 0x800) as u16;
        let length = (entry.length / 0x800) as u16;
        stream
            .write_all(&offset.to_le_bytes())
            .expect("Failed to write offset to header");
        stream
            .write_all(&length.to_le_bytes())
            .expect("Failed to write length to header");
    }

    // Writing data into the file...
    for entry in &file_entries {
        stream
            .seek(SeekFrom::Start(entry.offset as u64))
            .expect("Failed to seek in output file");
        stream
            .write_all(&entry.data)
            .expect("Failed to write data to output file");
    }

    stream
}
