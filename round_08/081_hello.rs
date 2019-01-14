fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // s1 has been moved

    //println!("{}, world!", s1);
    println!("s1 = {}, s2 = {}", s1, s2);
}
