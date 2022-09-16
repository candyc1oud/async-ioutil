use futures::AsyncReadExt;
use std::io::Result;
use std::marker::Unpin;

/// Read all data into `Vec<u8>`.
pub async fn read_all<R: AsyncReadExt + Unpin>(reader: &mut R) -> Result<Vec<u8>> {
    let mut bytes: Vec<u8> = Vec::new();
    reader.read_to_end(&mut bytes).await?;
    Ok(bytes)
}
