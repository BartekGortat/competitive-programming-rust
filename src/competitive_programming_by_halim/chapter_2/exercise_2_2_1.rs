use std::collections::{BTreeMap, HashSet};
use std::hash::Hash;

pub fn test_1_contains_duplicate<T>(vals: &Vec<T>) -> bool
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

pub fn task_2_find_pair(vals: &Vec<i32>, sum: &i32) -> Option<(i32, i32)> {
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

fn task_3_find_index(vals: &Vec<i32>, key: i32) -> Option<usize> {
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

fn task_4_sorted_in_range(vals: &Vec<i32>, inclusive_range: (i32, i32)) -> Vec<i32> {
    let mut included_vals: BTreeMap<i32, i32> = BTreeMap::new();
    for val_ref in vals {
        let val = *val_ref;
        if inclusive_range.0 <= val && val <= inclusive_range.1 {
            let count = included_vals.get(val_ref);
            let default = 0;
            let incremented_count = *count.unwrap_or(&default) + 1;
            included_vals.insert(val, incremented_count);
        }
    }

    let mut sorted_vals: Vec<i32> = Vec::new();
    for (val, count) in included_vals {
        for _ in 0..count {
            sorted_vals.push(val);
        }
    }
    sorted_vals
}

fn task_5_count_longest_increasing_subarray_len(vals: &Vec<i32>) -> i32 {
    let mut max_len = 0;
    let mut len = 1;
    let mut last_element = i32::MIN;
    for val_ref in vals {
        let val = *val_ref;
        if val > last_element {
            len += 1;
            last_element = val;
        } else {
            if len > max_len {
                max_len = len;
            }
            len = 1;
            last_element = val;
        }
    }
    if len > max_len {
        max_len = len;
    }
    return max_len;
}

fn task_6_find_median(vals: &Vec<i32>) -> i32 {
    let mut copied_vals: Vec<i32> = vals.iter().map(|val| { *val }).collect();
    copied_vals.sort();
    return copied_vals[copied_vals.len() / 2];
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn task_1_test_contains_duplicate_when_no_duplicate() {
        // given
        let vals = vec![4, 7, 3, 6, 1, 5, 8];
        let expected_result = false;

        // when
        let actual_result = test_1_contains_duplicate(&vals);

        // then
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn task_1_test_contains_duplicate_when_duplicate() {
        // given
        let vals = vec![4, 7, 3, 6, 1, 4, 8];
        let expected_result = true;

        // when
        let actual_result = test_1_contains_duplicate(&vals);

        // then
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn task_2_test_find_pair_when_pair_present() {
        // given
        let vals = vec![3, 6, 2, 7, 11];
        let sum = 5;
        let expected_result = Some((2, 3));

        // when
        let actual_result = task_2_find_pair(&vals, &sum);

        // then
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn task_2_test_find_pair_when_pair_absent() {
        // given
        let vals = vec![3, 6, 2, 7, 11];
        let sum = 50;
        let expected_result = None;

        // when
        let actual_result = task_2_find_pair(&vals, &sum);

        // then
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn task_3_test_find_index_when_key_in_middle_and_even_space() {
        // given
        let vals = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let key = 5;
        let expected_result = Some(4);

        // when
        let actual_result = task_3_find_index(&vals, key);

        // then
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn task_3_test_find_index_when_key_in_middle_and_odd_space() {
        // given
        let vals = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let key = 5;
        let expected_result = Some(4);

        // when
        let actual_result = task_3_find_index(&vals, key);

        // then
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn task_3_test_find_index_when_key_in_left_corner() {
        // given
        let vals = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let key = 1;
        let expected_result = Some(0);

        // when
        let actual_result = task_3_find_index(&vals, key);

        // then
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn task_3_test_find_index_when_key_in_right_corner() {
        // given
        let vals = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let key = 8;
        let expected_result = Some(7);

        // when
        let actual_result = task_3_find_index(&vals, key);

        // then
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn task_3_test_find_index_when_key_second_left_to_middle() {
        // given
        let vals = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let key = 4;
        let expected_result = Some(3);

        // when
        let actual_result = task_3_find_index(&vals, key);

        // then
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn task_3_test_find_index_when_key_second_right_to_middle() {
        // given
        let vals = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let key = 6;
        let expected_result = Some(5);

        // when
        let actual_result = task_3_find_index(&vals, key);

        // then
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn task_4_test_sort_in_range() {
        // given
        let vals = vec![6, 3, 9, 4, 1, 7, 9, 4, 8];
        let range = (4, 8);
        let expected_sorted_vals = vec![4, 4, 6, 7, 8];

        // when
        let actual_sorted_vals = task_4_sorted_in_range(&vals, range);

        // then
        assert_eq!(actual_sorted_vals, expected_sorted_vals);
    }

    #[test]
    fn task_5_test_count_longest_increasing_subarray_len() {
        // given
        let vals = vec![4, 5, 7, 3, 1, 2, 5, 6, 7, 4, 2, 3, 6];
        let expected_len = 5;

        // when
        let actual_len = task_5_count_longest_increasing_subarray_len(&vals);

        // then
        assert_eq!(actual_len, expected_len);
    }

    #[test]
    fn task_6_test_find_median() {
        // given
        let vals = vec![9, 1, 5, 7, 2, 4, 6, 6, 2, 3, 1];
        let expected_median = 4;

        // when
        let actual_median = task_6_find_median(&vals);

        // then
        assert_eq!(actual_median, expected_median);
    }
}