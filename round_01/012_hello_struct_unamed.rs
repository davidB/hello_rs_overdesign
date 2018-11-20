struct Greeter(String);

fn main() {
    let greeter = Greeter("David");
    println!(greeter.0);
    //println!("Hello, {}", greeter.0);
    //println!("Hello, {}", greeter);
}
