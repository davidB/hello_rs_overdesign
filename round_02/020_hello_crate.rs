//! ```cargo
//! [dependencies]
//! time = "0.1.25"
//! ```
extern crate time;

fn main() {
    println!("{}", time::now().rfc822z());
}
