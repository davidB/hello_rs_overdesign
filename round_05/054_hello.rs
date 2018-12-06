use std::fs;

fn main() {
    let val = fs::read_to_string("hello.txt")?;
    println!("{}", val);
}
