struct Greeter(String);

fn main() {
    let greeter = Greeter("David")
    println!("Hello, {}", greeter.0);
}
