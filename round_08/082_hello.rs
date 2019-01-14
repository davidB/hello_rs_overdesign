// takes_and_gives_back will take a String and return one
fn takes_ownership(a_string: String){
    println!("{}", a_string);
}

fn main() {
    let s1 = String::from("hello");
    takes_ownership(s1);
    println!("s1 = {}", s1);
}
