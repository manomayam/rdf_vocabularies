//! This module provides helpers to handle io

use std::{
    fs::{self, File},
    io::{BufWriter, Write},
    path::Path,
};

use anyhow::Context;

/// writes content to file at given path
pub fn write_to_file(content: &str, file_path: &Path) -> anyhow::Result<()> {
    BufWriter::new(
        File::create(file_path)
            .with_context(|| format!("error in creating file {}", file_path.to_string_lossy()))?,
    )
    .write(content.as_bytes())
    .with_context(|| format!("error in writing to file {}", file_path.to_string_lossy()))?;
    Ok(())
}

/// reads content as string from file at given path
pub fn read_from_file(file_path: &Path) -> anyhow::Result<String> {
    Ok(fs::read_to_string(file_path)
        .with_context(|| format!("error in reading file {}", file_path.to_string_lossy()))?)
}
