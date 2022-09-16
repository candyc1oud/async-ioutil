use ioutils::read::*;
use std::io::Result;

#[test]
fn test_read_all() -> Result<()> {
    let mut f = std::fs::File::open("Cargo.toml")?;
    read_all(&mut f)?;
    Ok(())
}
