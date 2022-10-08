#![allow(clippy::cast_sign_loss)]

use std::collections::{HashMap, HashSet};

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
#[must_use]
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
#[must_use]
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
/// See [`bifurcate_by`] for `partition` usage example.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// # use dryip::arrays::bifurcate;
///let expected: Vec<Vec<&str>> = [["beep", "boop", "bar"].to_vec(), ["foo"].to_vec()].to_vec();
///assert_eq!(
///    expected,
///    bifurcate(
///      &["beep", "boop", "foo", "bar"].to_vec(),
///      &[true, true, false, true].to_vec()
///    )
///);
/// ```
#[must_use]
pub fn bifurcate<'a>(lst: &'a [&str], filter: &'a [bool]) -> Vec<Vec<&'a str>> {
    let result1: Vec<&str> = lst
        .iter()
        .zip(filter)
        .filter(|(_, lst2)| **lst2)
        .map(|(lst1, _)| *lst1)
        .collect();

    let result2: Vec<&str> = lst
        .iter()
        .zip(filter)
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
///let expected: Vec<Vec<&str>> = [["beep", "boop", "bar"].to_vec(), ["foo"].to_vec()].to_vec();
///assert_eq!(
///    expected,
///    bifurcate_by(&["beep", "boop", "foo", "bar"].to_vec(), &starts_with)
///);
/// ```
#[must_use]
pub fn bifurcate_by<'a>(lst: &[&'a str], f: &'a dyn Fn(&str) -> bool) -> Vec<Vec<&'a str>> {
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
///assert_eq!(expected, chunk(&input, 2));
/// ```
#[must_use]
pub fn chunk(lst: &[i32], size: usize) -> Vec<Vec<i32>> {
    lst.chunks(size).into_iter().map(<[i32]>::to_vec).collect()
}

/// Removes falsey values from a list.
///
/// We have `Option` type in Rust. This is the recomended type to use
/// in this case.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// # use dryip::arrays::compact;
///let input = vec![None, Some(1), None, None];
///let expected = vec![Some(1)];
///assert_eq!(expected, compact(input));
/// ```
#[must_use]
pub fn compact(lst: Vec<Option<i32>>) -> Vec<Option<i32>> {
    lst.into_iter().filter(<Option<i32>>::is_some).collect()
}

/// Groups the elements of a list based on the given function.
///
/// Groups the elements of a list based on the given function and returns the count of elements in each group.
///
/// The function signature is similar to [`bifurcate_by`].
/// It uses `HashMap`'s `entry()` to update the previous result.
///
/// # Examples
///
/// Basic usage:
/// ```
/// # use std::collections::HashMap;
/// # use dryip::arrays::count_by;
///fn len(item: &str) -> i32 {
///    item.len() as i32
///}
///let expected = HashMap::from([(3, 2), (5, 1)]);
///let input = vec!["one", "two", "three"];
///assert_eq!(expected, count_by(input, &len));
///```
pub fn count_by(lst: Vec<&str>, f: &dyn Fn(&str) -> i32) -> HashMap<i32, i32> {
    let mut result = HashMap::new();
    for item in lst {
        let prev = result.entry(f(item)).or_insert(0);
        *prev += 1;
    }
    result
}

/// Counts occurrences
///
/// Counts the occurrences of a value in a vector.
///
/// Increment a counter for every item in the vector that has the given value and is of the same type.
/// Using [`std::iter::Iterator::fold`] will make our code shorter.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// # use dryip::arrays::count_occurrences;
/// let input = vec![1, 1, 2, 1, 2, 3];
/// assert_eq!(3, count_occurrences(&input, 1));
/// ```
///
/// # Alternatives:
///
/// Using [`std::iter::Iterator::count`]
///
/// ```rust
/// fn count_occurrences(lst: &[i32], val: i32) -> i32 {
///    lst.iter().filter(|&&x| x == val).count() as i32
/// }
///
/// let input = vec![1, 1, 2, 1, 2, 3];
/// assert_eq!(3, count_occurrences(&input, 1));
/// ```
#[must_use]
pub fn count_occurrences(lst: &[i32], val: i32) -> i32 {
    lst.iter()
        .fold(0, |acc, &x| if x == val { acc + 1 } else { acc })
}

/// Deep flattens a list.
///
/// `slice` has a built in [`std::iter::Iterator::flatten`]
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// # use dryip::arrays::deep_flatten;
/// let input = vec![vec![1], vec![2], vec![3, 4], vec![5]];
/// let expected = vec![1, 2, 3, 4, 5];
/// assert_eq!(expected, deep_flatten(input));
/// ```
#[must_use]
pub fn deep_flatten(lst: Vec<Vec<i32>>) -> Vec<i32> {
    lst.into_iter().flatten().collect()
}

/// Difference
///
/// Returns the difference between two iterables.
///
/// - Convert b slice to [`use std::collections::HashSet`] to remove duplicates.
/// - Iterate over a and keep the values not contained in b using [`std::iter::Iterator::filter`]
#[must_use]
pub fn difference(a: &[i32], b: &[i32]) -> Vec<i32> {
    let b: HashSet<_> = b.iter().copied().collect();
    a.iter().filter(|x| !b.contains(x)).copied().collect()
}

/// Every
///
/// Returns True if the provided function returns True for every element in the vector, False otherwise.
///
/// Use [`std::iter::Iterator::any`] to tests if any element of the iterator matches a predicate.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// # use dryip::arrays::every;
/// fn bigger(item: i32) -> bool {
///     item > 1
/// }
///
/// let input = &[4, 2, 3];
/// assert_eq!(true, every(input, &bigger));
/// ````
pub fn every(lst: &[i32], f: &dyn Fn(i32) -> bool) -> bool {
    lst.iter().any(|&x| f(x))
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
/// assert_eq!([5, 10, 15, 20, 25].to_vec(), arithmetic_progression(5, 25));
/// ```
/// [Range]: https://doc.rust-lang.org/std/ops/struct.Range.html
#[must_use]
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
        let expected: Vec<Vec<&str>> = vec![vec!["beep", "boop", "bar"], vec!["foo"]];
        assert_eq!(
            expected,
            bifurcate(
                ["beep", "boop", "foo", "bar"].as_ref(),
                [true, true, false, true].as_ref()
            )
        );
    }
    #[test]
    fn test_bifurcate_by() {
        fn starts_with(item: &str) -> bool {
            item.starts_with('b')
        }

        let expected: Vec<Vec<&str>> = vec![vec!["beep", "boop", "bar"], vec!["foo"]];
        assert_eq!(
            expected,
            bifurcate_by(["beep", "boop", "foo", "bar"].as_ref(), &starts_with)
        );
    }
    #[test]
    fn test_chunk() {
        let input = vec![1, 2, 3, 4, 5];
        let expected = vec![vec![1, 2], vec![3, 4], vec![5]];
        assert_eq!(expected, chunk(&input, 2));

        let input = vec![1, 2, 3, 4, 5];
        let expected = vec![vec![1], vec![2], vec![3], vec![4], vec![5]];
        assert_eq!(expected, chunk(&input, 1));

        let input = vec![1, 2, 3, 4, 5];
        let expected = vec![vec![1, 2, 3, 4, 5]];
        assert_eq!(expected, chunk(&input, 5));
    }
    #[test]
    fn test_compact() {
        let input = vec![None, Some(1), None, None];
        let expected = vec![Some(1)];
        assert_eq!(expected, compact(input));
    }
    #[test]
    fn test_count_by() {
        fn len(item: &str) -> i32 {
            item.len() as i32
        }
        let expected = HashMap::from([(3, 2), (5, 1)]);
        let input = vec!["one", "two", "three"];
        assert_eq!(expected, count_by(input, &len));
    }
    #[test]
    fn test_count_occurences() {
        let input = vec![1, 1, 2, 1, 2, 3];
        assert_eq!(3, count_occurrences(&input, 1));

        let input = vec![0, 100, 2, 1, 2, 3];
        assert_eq!(1, count_occurrences(&input, 0));
    }
    #[test]
    fn test_deep_flatten() {
        let input = vec![vec![1], vec![2], vec![3, 4], vec![5]];
        let expected = vec![1, 2, 3, 4, 5];
        assert_eq!(expected, deep_flatten(input));

        let input = vec![vec![1, 2, 3, 4], vec![5, 6]];
        let expected = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(expected, deep_flatten(input));
    }
    #[test]
    fn test_difference() {
        let (a, b) = (&[1, 2, 3], &[1, 2, 4]);
        assert_eq!(vec![3], difference(a, b));
    }
    #[test]
    fn test_every() {
        fn bigger(item: i32) -> bool {
            item > 1
        }
        let input = &[4, 2, 3];
        assert_eq!(true, every(input, &bigger));
    }
    #[test]
    fn test_arithmetic_progression() {
        assert_eq!(vec![5, 10, 15, 20, 25], arithmetic_progression(5, 25));
        assert_eq!(vec![2, 4, 6, 8, 10], arithmetic_progression(2, 10));
    }
}
