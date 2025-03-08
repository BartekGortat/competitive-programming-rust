pub fn bubble(xs: &mut Vec<i32>) {
    let outer_bound = xs.len() as i32;
    for outer_index in 0 .. outer_bound {
        let inner_bound = outer_bound as i32 - outer_index - 1;
        for inner_index in 0 .. inner_bound  {
            let left_index = inner_index as usize;
            let right_index = left_index + 1;
            if xs[left_index] > xs[right_index] {
                xs.swap(left_index, right_index);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::algorithm::sort::quadratic::bubble;

    #[test]
    fn test_bubble_on_empty() {
        // given
        let mut xs: Vec<i32> = Vec::with_capacity(0);
        let expected_xs: Vec<i32> = Vec::with_capacity(0);

        // when
        bubble(&mut xs);

        // then
        assert_eq!(xs, expected_xs);
    }

    #[test]
    fn test_bubble_on_single() {
        // given
        let mut xs: Vec<i32> = vec![5];
        let expected_xs: Vec<i32> = vec![5];

        // when
        bubble(&mut xs);

        // then
        assert_eq!(xs, expected_xs);
    }

    #[test]
    fn test_bubble_on_multiple() {
        // given
        let mut xs: Vec<i32> = vec![5, 1, 8, 4, 9, 3, 4, 2];
        let expected_xs: Vec<i32> = vec![1, 2, 3, 4, 4, 5, 8, 9];

        // when
        bubble(&mut xs);

        // then
        assert_eq!(xs, expected_xs);
    }
}