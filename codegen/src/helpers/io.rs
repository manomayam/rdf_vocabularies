use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use anyhow::Context;

pub fn write_to_file(content: &str, file_path: &Path) -> anyhow::Result<()> {
    BufWriter::new(
        File::create(file_path)
            .with_context(|| format!("error in creating file {}", file_path.to_string_lossy()))?,
    )
    .write(content.as_bytes())
    .with_context(|| format!("error in writing to file {}", file_path.to_string_lossy()))?;
    Ok(())
}
