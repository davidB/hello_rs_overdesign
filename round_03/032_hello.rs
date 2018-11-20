fn find_env_var(str: &str) -> Option<String> {
    None
}

fn main() {
    println!("SHELL = {}", find_env_var("SHELL"));
}

// ; debug, unwrap..., expect, Some, map, or, or_else
