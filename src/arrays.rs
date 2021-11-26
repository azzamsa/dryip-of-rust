use std::collections::HashSet;

/// Check if all value in given list are equal.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use thirtyseconds::arrays::all_equal;
///
/// assert!(all_equal(vec![2, 2, 2]));
/// ```
pub fn all_equal(lst: Vec<u32>) -> bool {
    // don't compare the list as is. only part of it.
    lst[1..] == lst[..lst.len() - 1]
}

/// Check if all value in given list are unique.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use thirtyseconds::arrays::all_unique;
///
/// assert!(all_unique(vec![1, 2, 3]));
/// ```
pub fn all_unique(lst: Vec<u32>) -> bool {
    lst.len() == lst.into_iter().collect::<HashSet<u32>>().len()
}

/// Returns a list of numbers in the arithmetic progression starting with the
/// given positive integer and up to the specified limit.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use thirtyseconds::arrays::find_multiples;
///
/// assert_eq!(vec![5, 10, 15, 20, 25], find_multiples(5, 25));
/// ```
pub fn find_multiples(n: i32, lim: i32) -> Vec<i32> {
    (n..lim + 1).step_by(n as usize).collect::<Vec<i32>>()
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
    fn test_find_multiples() {
        assert_eq!(vec![5, 10, 15, 20, 25], find_multiples(5, 25));
        assert_eq!(vec![2, 4, 6, 8, 10], find_multiples(2, 10));
    }
}
