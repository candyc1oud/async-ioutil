use ioutils::read::*;
use std::io::Result;

#[async_std::test]
async fn test_read_all() -> Result<()> {
    let mut f = async_std::fs::File::open("Cargo.toml").await?;
    read_all(&mut f).await?;
    Ok(())
}
