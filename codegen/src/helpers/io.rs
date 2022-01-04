use std::{path::Path, io::{BufWriter, Write}, fs::File};

pub fn write_to_file(content: &str, file_path: &Path) {
    BufWriter::new(
        File::create(file_path).unwrap()
    ).write(content.as_bytes()).unwrap();
}
