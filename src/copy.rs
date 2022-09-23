use futures::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use std::io::Result;
use std::marker::Unpin;

/// Copy data from reader to writer.
pub async fn copy_buffer<R, W>(reader: &mut R, writer: &mut W, buffer: &mut [u8]) -> Result<usize>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    let mut n = 0;
    loop {
        let nn = reader.read(&mut buffer[..]).await?;
        if nn == 0 {
            break;
        }
        n += nn;
        writer.write_all(&buffer[..nn]).await?;
    }
    Ok(n)
}

/// Copy data from reader to writer.
pub async fn copy<R, W>(reader: &mut R, writer: &mut W) -> Result<usize>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    const BUF_SIZE: usize = 32 * 1024;
    let mut buffer = [0u8; BUF_SIZE];
    let n = copy_buffer(reader, writer, &mut buffer).await?;
    Ok(n)
}
