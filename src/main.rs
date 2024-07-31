mod extract;

use std::{env, io};
use std::path::Path;

use crate::extract::{read_file, process_entries};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }

    let file_path = Path::new(&args[1]);

    let stream = read_file(&file_path)?;
    let entries = process_entries(stream);

    for entry in entries {
        println!("Entry {{ offset: 0x{:08X}, length: 0x{:08X} }}", entry.offset, entry.length);
    }

    Ok(())
}
