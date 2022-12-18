use ethers::prelude::Abigen;
use std::error::Error as Err;
fn main() -> Result<(), Box<dyn Err>> {
    Abigen::new("StdEvent", "./contracts/ASTEvent.json")?
        .generate()?
        .write_to_file("./src/std_events.rs")?;
    Ok(())
}
