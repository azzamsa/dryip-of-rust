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
/// See `bifurcate_by` for `partition_example`
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// # use dryip::arrays::bifurcate;
///let result: Vec<Vec<&str>> = [["beep", "boop", "bar"].to_vec(), ["foo"].to_vec()].to_vec();
///assert_eq!(
///    result,
///    bifurcate(
///      ["beep", "boop", "foo", "bar"].to_vec(),
///      [true, true, false, true].to_vec()
///    )
///);
/// ```

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

/// Splits values into two groups according to a passed function.
///
/// Which specifies which group an element in the input list belongs to.
/// If the function returns True, the element belongs to the first group; otherwise, it belongs to the second group.
///
/// Use list comprehension to add elements to groups, based on fn.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// # use dryip::arrays::bifurcate_by;
///fn starts_with(item: &str) -> bool {
///      item.starts_with('b')
///}
///
///let result: Vec<Vec<&str>> = [["beep", "boop", "bar"].to_vec(), ["foo"].to_vec()].to_vec();
///assert_eq!(
///    result,
///    bifurcate_by(["beep", "boop", "foo", "bar"].to_vec(), &starts_with)
///);
/// ```

pub fn bifurcate_by<'a>(lst: Vec<&'a str>, f: &'a dyn Fn(&str) -> bool) -> Vec<Vec<&'a str>> {
    let (result, result1): (Vec<&str>, Vec<&str>) = lst.iter().partition(|item| f(item));

    [result, result1].to_vec()
}

/// Chunks a list into smaller lists of a specified size.
///
/// Rust `slice` has built-in `chucks()` function.
/// But it returns a slice. So you need to iterate the slice and convert
/// them into a vector.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// # use dryip::arrays::chunk;
///let input = vec![1, 2, 3, 4, 5];
///let expected = vec![vec![1, 2], vec![3, 4], vec![5]];
///assert_eq!(expected, chunk(input, 2));
/// ```
pub fn chunk(lst: Vec<i32>, size: usize) -> Vec<Vec<i32>> {
    lst.chunks(size)
        .into_iter()
        .map(|item| item.to_vec())
        .collect()
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
    fn test_bifurcate_by() {
        fn starts_with(item: &str) -> bool {
            item.starts_with('b')
        }

        let result: Vec<Vec<&str>> = [["beep", "boop", "bar"].to_vec(), ["foo"].to_vec()].to_vec();
        assert_eq!(
            result,
            bifurcate_by(["beep", "boop", "foo", "bar"].to_vec(), &starts_with)
        );
    }
    #[test]
    fn test_chunk() {
        let input = vec![1, 2, 3, 4, 5];
        let expected = vec![vec![1, 2], vec![3, 4], vec![5]];
        assert_eq!(expected, chunk(input, 2));

        let input = vec![1, 2, 3, 4, 5];
        let expected = vec![vec![1], vec![2], vec![3], vec![4], vec![5]];
        assert_eq!(expected, chunk(input, 1));

        let input = vec![1, 2, 3, 4, 5];
        let expected = vec![vec![1, 2, 3, 4, 5]];
        assert_eq!(expected, chunk(input, 5));
    }
    #[test]
    fn test_arithmetic_progression() {
        assert_eq!(vec![5, 10, 15, 20, 25], arithmetic_progression(5, 25));
        assert_eq!(vec![2, 4, 6, 8, 10], arithmetic_progression(2, 10));
    }
}
