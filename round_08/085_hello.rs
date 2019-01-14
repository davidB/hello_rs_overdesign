fn takes_rw_ref(s: &mut String) {
    s.push_str(", world!");
    println!("{}", s);
}

fn main() {
    let mut s1 = String::from("hello");
    takes_rw_ref(&mut s1);
    println!("s1 = {}", s1);
}
