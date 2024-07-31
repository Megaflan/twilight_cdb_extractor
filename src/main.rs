mod extract;

use std::fs::File;
use std::io::Write;
use std::{env, fs, io};
use std::path::Path;

use crate::extract::{read_file, process_entries};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <mode> <file>", args[0]);
        eprintln!("Modes:");
        eprintln!("-e: Extract the CDB file.");
        //eprintln!("-ce: Extract and decompress the CDB file.");
        //eprintln!("-i: Repack the CDB file.");
        //eprintln!("-ci: Repack and compress the CDB file.");
        std::process::exit(1);
    }

    let file_path = Path::new(&args[2]);
    let mode = &args[1];

    let stream = read_file(&file_path)?;

    let compressed: bool = mode.contains("-ce") || mode.contains("-ci");
    let entries = process_entries(stream, compressed);

    //Write files (Temporal)
    let filename_no_ext = file_path.file_stem().unwrap().to_str().unwrap();
    let dir_path = Path::new(filename_no_ext);
    fs::create_dir_all(&dir_path).expect("Failed to create directory");

    for entry in entries {
        println!("Entry {{ offset: 0x{:08X}, length: 0x{:08X} }}", entry.offset, entry.length);
        
        let output_path = dir_path.join(format!("{}_{}.bin", filename_no_ext, entry.offset));
        let mut output_file = File::create(&output_path).expect("Failed to create output file");
        output_file.write_all(&entry.data).expect("Failed to write to output file");
    }

    Ok(())
}
