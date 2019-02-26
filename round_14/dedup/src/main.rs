use std::collections::BTreeSet;
use std::collections::HashSet;
use std::hash::Hash;

fn main() {
    dedup_via_hashset(&std::env::args().collect::<Vec<_>>()).iter().for_each(|e| println!("{:?}", e));
}

pub fn dedup_via_hashset<T>(l: &[T]) -> Vec<T> where T: Hash + Eq + Clone {
    let set: HashSet<_> = l.iter().cloned().collect();
    set.into_iter().collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn test_dedup() {
        let actual = dedup_via_hashset(&vec![-1,2,0,3,3,2]);
        let expected = vec![&-1,&2,&0,&3];
        assert_that!(&actual).has_length(expected.len());
        assert_that!(&actual).contains_all_of(&expected);
    }


    #[test]
    fn test_dedup_empty() {
        let empty: Vec<i32> = Vec::new();
        let actual = dedup_via_hashset(&empty);
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}
