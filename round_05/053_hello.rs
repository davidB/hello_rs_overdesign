use std::fs;

fn main() {
    fs::read_to_string("hello.txt")
        .map(|x| x.to_uppercase())
        .and_then(|val| {
            println!("{}", val);
            Ok(val)
        }).or_else(|e| {
            println!("couldn't read: {}", e);
            Err(e)
        });
}
