pub fn task_1_calculate_remainder(s: i32, n: i32) -> i32 {
    let mask = n - 1;
    return s & mask;
}

pub fn task_2_is_power_of_two(s: i32) -> bool {
    let mask = s - 1;
    return s & mask == 0;
}

pub fn task_3_turn_of_last_bit(s: i32) -> i32 {
    let mask = s - 1;
    return s & mask;
}

pub fn task_4_turn_on_last_zero(s: i32) -> i32 {
    let mask = s + 1;
    return s | mask;
}

pub fn task_5_turn_off_last_consecutive_run_of_ones(s: i32) -> i32 {
    let mask = s + 1;
    return s & mask;
}

pub fn task_6_turn_on_last_consecutive_run_of_zeros(s: i32) -> i32 {
    let mask = s - 1;
    return s | mask;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn task_1_test_calculate_remainder() {
        // given
        let s = 7;
        let n = 4;
        let expected_result = 3;

        // when
        let actual_result = task_1_calculate_remainder(s, n);

        // then
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn task_2_test_is_power_of_two_when_power_of_two() {
        // given
        let s = 16;
        let expected_result = true;

        // when
        let actual_result = task_2_is_power_of_two(s);

        // then
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn task_2_test_is_power_of_two_when_not_power_of_two() {
        // given
        let s = 14;
        let expected_result = false;

        // when
        let actual_result = task_2_is_power_of_two(s);

        // then
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn task_3_test_turn_off_last_bit() {
        // given
        let s = 40;
        let expected_result = 32;

        // when
        let actual_result = task_3_turn_of_last_bit(s);

        // then
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn task_4_test_turn_on_last_zero() {
        // given
        let s = 41;
        let expected_result = 43;

        // when
        let actual_result = task_4_turn_on_last_zero(s);

        // then
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn task_5_test_turn_off_last_consecutive_run_of_ones() {
        // given
        let s = 39;
        let expected_result = 32;

        // when
        let actual_result = task_5_turn_off_last_consecutive_run_of_ones(s);

        // then
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn task_6_test_turn_on_last_consecutive_run_of_zeros() {
        // given
        let s = 36;
        let expected_result = 39;

        // when
        let actual_result = task_6_turn_on_last_consecutive_run_of_zeros(s);

        // then
        assert_eq!(actual_result, expected_result);
    }
}