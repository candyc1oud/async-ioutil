use std::io::{Read, Result};

/// Read all data into `Vec<u8>`.
pub fn read_all<R: Read>(reader: &mut R) -> Result<Vec<u8>> {
    let mut bytes: Vec<u8> = Vec::new();
    reader.read_to_end(&mut bytes)?;
    Ok(bytes)
}
