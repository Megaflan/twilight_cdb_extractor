pub fn decompress(stream: Vec<u8>) -> Vec<u8> {
    let mut pos = 0;
    let mut buffer: Vec<u8> = Vec::new();

    while pos < stream.len() {
        let value = stream[pos];
        pos += 1;

        match value {
            //Direct Copy
            0x00..=0x7F => {
                let length = value as usize;
                if pos + length <= stream.len() {
                    buffer.extend_from_slice(&stream[pos..pos + length]);
                    pos += length;
                }
            }
            //Zero Fill
            0x80..=0xBF => {                
                //Adapt RLE as needed [TODO]
            }
            //Back-Reference Copy
            0xC0..=0xDF => {
                //Adapt RLE as needed [TODO]
            }
            //Cumulative Sum Generation
            0xE0..=0xEF => {
                //Adapt RLE as needed [TODO]
            }
            _ => {}
        }
    }

    buffer
}


pub fn compress(data: Vec<u8>) -> Vec<u8> {
    // For now, we are going to send data without changes.
    data
}