fn takes_ro_ref(s: &String) {
    println!("{}, world!", s);
}

fn main() {
    let s1 = String::from("hello");
    takes_ro_ref(&s1);
    println!("s1 = {}", s1);
}
