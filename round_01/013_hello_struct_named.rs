struct Greeter {
    who: String,
}

fn main() {
    let greeter = Greeter {
        who: "David".into(),
    };
    println!("Hello, {}", greeter.who);
}
