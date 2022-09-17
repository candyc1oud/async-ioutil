use async_ioutil::copy::*;
use std::io::Result;

#[async_std::test]
async fn test_copy_buffer() -> Result<()> {
    let mut v: Vec<u8> = Vec::new();
    let mut f = async_std::fs::File::open("Cargo.toml").await?;
    let mut buffer = [0u8; 1024];
    copy_buffer(&mut f, &mut v, &mut buffer).await?;
    Ok(())
}
