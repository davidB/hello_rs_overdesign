fn dedup(l: &Vec<i32>) -> Vec<i32> {
    l.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dedup() {
        let actual = dedup(&vec![-1,2,0,3,3,2]);
        let expected = vec![-1,2,0,3];
        assert_eq!(expected, expected);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_dedup_empty() {
        let actual = dedup(&vec![]);
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}
