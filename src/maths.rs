#![allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]

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
/// assert_eq!(2, average(&[1, 2, 3].to_vec()));
/// ```
#[must_use]
pub fn average(nums: &[i32]) -> i32 {
    nums.iter().sum::<i32>() / nums.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average() {
        assert_eq!(2, average(&[1, 2, 3]));
        assert_eq!(4, average(&[2, 4, 8]));
    }
}
