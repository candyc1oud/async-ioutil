use async_ioutil::Discard;
use futures::{AsyncReadExt, AsyncWriteExt};
use std::io::Result;

#[async_std::test]
async fn test_discard() -> Result<()> {
    let mut rw = Discard::new();
    rw.write(&[0x01, 0x02, 0x06]).await?;
    let mut v = [0u8; 512];
    rw.read(&mut v).await?;
    Ok(())
}
