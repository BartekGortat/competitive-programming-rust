use std::collections::HashSet;
use std::hash::Hash;

pub fn contains_duplicate<T>(vals: &Vec<T>) -> bool
where
    T: Eq,
    T: Hash
{
    let mut visited_vals: HashSet<&T> = HashSet::new();
    let iter = vals.iter();
    for val in iter {
        if visited_vals.contains(val) {
            return true;
        }
        visited_vals.insert(val);
    }
    false
}

pub fn find_pair(vals: &Vec<i32>, sum: &i32) -> Option<(i32, i32)> {
    let mut visited_vals: HashSet<&i32> = HashSet::new();
    let iter = vals.iter();
    for x in iter {
        let y = sum - x;
        if visited_vals.contains(&y) {
            return Some((*x, y));
        }
        visited_vals.insert(x);
    }
    None
}

fn find_index(vals: &Vec<i32>, key: i32) -> Option<usize> {
    let mut left_index = 0;
    let mut right_index = vals.len();
    while left_index < right_index {
        let mid_index = left_index + (right_index - left_index) / 2;
        let mid_val = vals[mid_index];
        if mid_val < key {
            left_index = mid_index;
        } else if mid_val > key {
            right_index = mid_index;
        } else {
            return Some(mid_index);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_contains_duplicate_when_no_duplicate() {
        // given
        let vals = vec![4, 7, 3, 6, 1, 5, 8];
        let expected_result = false;

        // when
        let actual_result = contains_duplicate(&vals);

        // then
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_contains_duplicate_when_duplicate() {
        // given
        let vals = vec![4, 7, 3, 6, 1, 4, 8];
        let expected_result = true;

        // when
        let actual_result = contains_duplicate(&vals);

        // then
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_find_pair_when_pair_present() {
        // given
        let vals = vec![3, 6, 2, 7, 11];
        let sum = 5;
        let expected_result = Some((2, 3));

        // when
        let actual_result = find_pair(&vals, &sum);

        // then
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_find_pair_when_pair_absent() {
        // given
        let vals = vec![3, 6, 2, 7, 11];
        let sum = 50;
        let expected_result = None;

        // when
        let actual_result = find_pair(&vals, &sum);

        // then
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_find_index_when_key_in_middle_and_even_space() {
        // given
        let vals = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let key = 5;
        let expected_result = Some(4);

        // when
        let actual_result = find_index(&vals, key);

        // then
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_find_index_when_key_in_middle_and_odd_space() {
        // given
        let vals = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let key = 5;
        let expected_result = Some(4);

        // when
        let actual_result = find_index(&vals, key);

        // then
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_find_index_when_key_in_left_corner() {
        // given
        let vals = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let key = 1;
        let expected_result = Some(0);

        // when
        let actual_result = find_index(&vals, key);

        // then
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_find_index_when_key_in_right_corner() {
        // given
        let vals = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let key = 8;
        let expected_result = Some(7);

        // when
        let actual_result = find_index(&vals, key);

        // then
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_find_index_when_key_second_left_to_middle() {
        // given
        let vals = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let key = 4;
        let expected_result = Some(3);

        // when
        let actual_result = find_index(&vals, key);

        // then
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_find_index_when_key_second_right_to_middle() {
        // given
        let vals = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let key = 6;
        let expected_result = Some(5);

        // when
        let actual_result = find_index(&vals, key);

        // then
        assert_eq!(actual_result, expected_result);
    }
}