//! Day 001: find the largest observed latency and its earliest index.

/// Return `(index, value)` for the first occurrence of the maximum.
/// Return `None` when the input is empty. Do not sort or mutate the input.
pub fn max_with_index(_values: &[i32]) -> Option<(usize, i32)> {
    todo!("First write your reasoning in days/day001/notes.md, then implement")
}

#[cfg(test)]
mod tests {
    use super::max_with_index;

    #[test]
    #[ignore = "Unsolved Day 001 exercise"]
    fn empty_input() {
        assert_eq!(max_with_index(&[]), None);
    }

    #[test]
    #[ignore = "Unsolved Day 001 exercise"]
    fn single_value() {
        assert_eq!(max_with_index(&[42]), Some((0, 42)));
    }

    #[test]
    #[ignore = "Unsolved Day 001 exercise"]
    fn largest_in_middle_and_input_unchanged() {
        let values = [12, 80, 25];
        assert_eq!(max_with_index(&values), Some((1, 80)));
        assert_eq!(values, [12, 80, 25]);
    }

    #[test]
    #[ignore = "Unsolved Day 001 exercise"]
    fn first_of_repeated_maxima() {
        assert_eq!(max_with_index(&[9, 3, 9]), Some((0, 9)));
    }

    #[test]
    #[ignore = "Unsolved Day 001 exercise"]
    fn negative_and_extreme_values() {
        assert_eq!(max_with_index(&[-8, -2, -7]), Some((1, -2)));
        assert_eq!(max_with_index(&[i32::MIN, i32::MAX]), Some((1, i32::MAX)));
    }
}
