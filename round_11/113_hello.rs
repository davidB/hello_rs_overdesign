
//! ```cargo
//! [dev-dependencies]
//! spectral = "0.6.0"
//! ```
#[cfg(test)]
extern crate spectral;

fn dedup(l: &Vec<i32>) -> Vec<i32> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn test_dedup() {
        let actual = dedup(&vec![-1,2,0,3,3,2]);
        let expected = vec![&-1,&2,&0,&3];
        assert_that!(&actual).has_length(expected.len());
        assert_that!(&actual).contains_all_of(&expected);
    }

    #[test]
    fn test_dedup_empty() {
        let actual = dedup(&vec![]);
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}
