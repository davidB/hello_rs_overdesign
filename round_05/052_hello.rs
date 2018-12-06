use std::env;
use std::fs;

fn main() {
    let msg = fs::read_to_string("hello.txt");
    match msg {
        Ok(val) => println!("{}", val),
        Err(e) => println!("couldn't read: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env_var() {
        let v = env::var("SHELL");
        assert_eq!(v.is_ok(), true);
        assert_eq!(v.is_err(), false);
        assert_eq!(v.clone(), Ok("/bin/zsh".to_owned()));
        assert_eq!(v.unwrap(), "/bin/zsh");
    }
}
