fn find_env_var(str: &str) -> Option<String> {
    None
}

fn main() {
    match find_env_var("SHELL") {
        None => println!("SHELL is undefined"),
        Some(_) => println!("SHELL is defined"),
    }
}
