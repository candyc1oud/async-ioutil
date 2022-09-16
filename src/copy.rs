use futures::{AsyncReadExt, AsyncWriteExt};
use std::io::Result;
use std::marker::Unpin;

// Copy data from reader to writer.
pub async fn copy_buffer<R: AsyncReadExt + Unpin, W: AsyncWriteExt + Unpin>(
    reader: &mut R,
    writer: &mut W,
    buffer: &mut [u8],
) -> Result<usize> {
    let mut n = 0;
    loop {
        let nn = reader.read(&mut buffer[..]).await?;
        if n == 0 {
            break;
        }
        n += nn;
        writer.write_all(&buffer[..nn]).await?;
    }
    Ok(n)
}
