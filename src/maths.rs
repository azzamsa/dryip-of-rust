/// Calculate the average of two or more numbers.
///
/// Use `sum()` to sum all of the args provided, divide by `args.len()`.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// # use dryip::maths::average;
/// assert_eq!(2.0, average(vec![1, 2, 3]));
/// ```
pub fn average(nums: Vec<u32>) -> f32 {
    (nums.iter().sum::<u32>() / nums.len() as u32) as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average() {
        assert!((average(vec![1, 2, 3]) - 2.0).abs() < f32::EPSILON);
        assert!((average(vec![2, 4, 8]) - 4.0).abs() < f32::EPSILON);
    }
}
