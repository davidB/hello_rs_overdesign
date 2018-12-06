use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let val = fs::read_to_string("hello0.txt")?;
    println!("{}", val);
    Ok(())
}
