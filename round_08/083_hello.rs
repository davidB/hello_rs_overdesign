// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope

    a_string  // a_string is returned and moves out to the calling function
}

fn main() {
    let mut s1 = String::from("hello");
    s1 = takes_and_gives_back(s1);
    //let s2 = takes_and_gives_back(s1);
    println!("s1 = {}", s1);
}
