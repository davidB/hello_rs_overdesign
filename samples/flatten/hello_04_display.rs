// https://doc.rust-lang.org/std/fmt/trait.Display.html
// use std::fmt;

struct Greeter {
    who: String,
}

// impl fmt::Display for Greeter {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.who)
//     }
// }

fn main() {
    let greeter = Greeter {
        who: "David".into(),
    };
    println!("Hello, {}", greeter);
}
