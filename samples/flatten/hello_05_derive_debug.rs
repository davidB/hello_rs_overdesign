#[derive(Debug, Clone)]
struct Greeter {
    who: String,
}

fn main() {
    let greeter = Greeter { who: "David" };
    println!("Hello, {:?}", greeter.who);
}
