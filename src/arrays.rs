use std::collections::HashSet;

/// Check if all value in given list are equal.
///
/// Use `HashSet` to eliminate duplicate elements and then use len() to check if length is 1.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// # use dryip::arrays::all_equal;
/// assert!(all_equal(vec![2, 2, 2]));
/// ```
pub fn all_equal(lst: Vec<u32>) -> bool {
    lst.into_iter().collect::<HashSet<u32>>().len() == 1
}

/// Check if all value in given list are unique.
///
/// - Use `HashSet` on the given vector to keep only unique occurrences.
/// - Use len() to compare the length of the unique values to the original list.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// # use dryip::arrays::all_unique;
/// assert!(all_unique(vec![1, 2, 3]));
/// ```
pub fn all_unique(lst: Vec<u32>) -> bool {
    lst.len() == lst.into_iter().collect::<HashSet<u32>>().len()
}

/// Splits values into two groups.
///
/// If an element in `filter` is `True`, the corresponding element in the collection belongs to the first group.
/// Otherwise, it belongs to the second group.
///
/// There are two options to use here. The first is using `enumerate()` and the second is `zip()`.
/// Both should have similar code.
///
/// The other option is `partition`. However it can't work with multiple input such the current case.
pub fn bifurcate(lst: Vec<&str>, filter: Vec<bool>) -> Vec<Vec<&str>> {
    let result1: Vec<&str> = lst
        .iter()
        .zip(&filter)
        .filter(|(_, lst2)| **lst2)
        .map(|(lst1, _)| *lst1)
        .collect();

    let result2: Vec<&str> = lst
        .iter()
        .zip(&filter)
        .filter(|(_, lst2)| !**lst2)
        .map(|(lst1, _)| *lst1)
        .collect();

    [result1, result2].to_vec()
}

/// Returns a list of numbers in the arithmetic progression starting with the
/// given positive integer and up to the specified limit.
///
/// Use [Range] to and increment it using `step_by()`
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// # use dryip::arrays::arithmetic_progression;
/// assert_eq!(vec![5, 10, 15, 20, 25], arithmetic_progression(5, 25));
/// ```
/// [Range]: https://doc.rust-lang.org/std/ops/struct.Range.html
pub fn arithmetic_progression(n: i32, lim: i32) -> Vec<i32> {
    (n..=lim).step_by(n as usize).collect::<Vec<i32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_equal() {
        assert!(all_equal(vec![1, 1, 1, 1]));
        assert!(!all_equal(vec![1, 2, 3, 4, 5, 6]));
    }
    #[test]
    fn test_all_unique() {
        assert!(all_unique(vec![1, 2, 3, 4, 5, 6]));
        assert!(!all_unique(vec![1, 2, 2, 3, 4, 5]));
    }
    #[test]
    fn test_bifurcate() {
        let result: Vec<Vec<&str>> = [["beep", "boop", "bar"].to_vec(), ["foo"].to_vec()].to_vec();
        assert_eq!(
            result,
            bifurcate(
                ["beep", "boop", "foo", "bar"].to_vec(),
                [true, true, false, true].to_vec()
            )
        );
    }
    #[test]
    fn test_arithmetic_progression() {
        assert_eq!(vec![5, 10, 15, 20, 25], arithmetic_progression(5, 25));
        assert_eq!(vec![2, 4, 6, 8, 10], arithmetic_progression(2, 10));
    }
}
