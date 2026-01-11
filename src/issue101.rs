// Rust Bytes Challenge Issue #101 Group String Rotations

trait CyclicalEqExt {
    fn cyclical_eq(&self, other: &str) -> bool;
}

impl CyclicalEqExt for String {
    fn cyclical_eq(&self, other: &str) -> bool {
        let mut right: VecDeque<u8> = self.as_bytes().to_vec().into();
        let left: VecDeque<u8> = other.as_bytes().to_vec().into();
        for _i in 0..left.len() {
            if left == right {
                return true;
            }
            let last = right.pop_back().unwrap();
            right.push_front(last);
        }
        false
    }
}

use std::collections::VecDeque;

pub fn group_rotations(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut result: Vec<Vec<String>> = Vec::new();
    'outer: for i in &strs {
        for j in result.iter_mut() {
            let first = j.first().unwrap();
            if i.cyclical_eq(first) {
                j.push(i.clone());
                continue 'outer;
            }
        }
        result.push(vec![i.clone()]);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted_groups(mut groups: Vec<Vec<String>>) -> Vec<Vec<String>> {
        for group in groups.iter_mut() {
            group.sort();
        }
        groups.sort_by_key(|g| g[0].clone());
        groups
    }

    #[test]
    fn test_basic_rotations() {
        let input = vec![
            "abc".to_string(),
            "bca".to_string(),
            "cab".to_string(),
            "xyz".to_string(),
            "yzx".to_string(),
            "zxy".to_string(),
        ];
        let mut result = group_rotations(input);
        result = sorted_groups(result);
        let expected = sorted_groups(vec![
            vec!["abc".to_string(), "bca".to_string(), "cab".to_string()],
            vec!["xyz".to_string(), "yzx".to_string(), "zxy".to_string()],
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_no_rotations() {
        let input = vec!["hello".to_string(), "world".to_string()];
        let mut result = group_rotations(input);
        result = sorted_groups(result);
        let expected = sorted_groups(vec![vec!["hello".to_string()], vec!["world".to_string()]]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_duplicates() {
        let input = vec!["abc".to_string(), "abc".to_string(), "bca".to_string()];
        let mut result = group_rotations(input);
        result = sorted_groups(result);
        let expected = sorted_groups(vec![vec![
            "abc".to_string(),
            "abc".to_string(),
            "bca".to_string(),
        ]]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_single_char() {
        let input = vec!["a".to_string(), "a".to_string(), "b".to_string()];
        let mut result = group_rotations(input);
        result = sorted_groups(result);
        let expected = sorted_groups(vec![
            vec!["a".to_string(), "a".to_string()],
            vec!["b".to_string()],
        ]);
        assert_eq!(result, expected);
    }
}
