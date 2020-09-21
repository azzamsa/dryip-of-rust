use std::collections::HashSet;

/// Convert HEX into RGB value.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// assert_eq!(vec![255, 165, 1], thirtyseconds::hex_to_rgb(&"FFA501"));
/// ```
pub fn hex_to_rgb(hex: &str) -> Vec<i64> {
    let to_base16 = |x| i64::from_str_radix(x, 16);
    // MAYBE can I collect as tuple directly?
    [0, 2, 4]
        .iter()
        .map(|&x| to_base16(&hex[x..x + 2]).unwrap())
        .collect::<Vec<_>>()
}

/// Convert RGB into HEX value.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// assert_eq!("FFA501", thirtyseconds::rgb_to_hex(255, 165, 1));
/// ```
pub fn rgb_to_hex(r: i32, g: i32, b: i32) -> String {
    format!("{:02X}{:02X}{:02X}", r, g, b)
}

/// Check if all value in given list are equal.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// assert_eq!(true, thirtyseconds::all_equal(vec![2, 2, 2]));
/// ```
pub fn all_equal(lst: Vec<u32>) -> bool {
    // don't compare the list as is. only part of it.
    &lst[1..] == &lst[..lst.len() - 1]
}

/// Check if all value in given list are unique.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// assert_eq!(true, thirtyseconds::all_unique(vec![1, 2, 3]));
/// ```
pub fn all_unique(lst: Vec<u32>) -> bool {
    &lst.len() == &lst.into_iter().collect::<HashSet<u32>>().len()
}

/// Returns a list of numbers in the arithmetic progression starting with the
/// given positive integer and up to the specified limit.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// assert_eq!(vec![5, 10, 15, 20, 25], thirtyseconds::find_multiples(5, 25));
/// ```
pub fn find_multiples(n: i32, lim: i32) -> Vec<i32> {
    (n..lim + 1).step_by(n as usize).collect::<Vec<i32>>()
}

/// Calculate the average of two or more numbers.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// assert_eq!(2.0, thirtyseconds::average(vec![1, 2, 3]));
/// ```
pub fn average(nums: Vec<u32>) -> f32 {
    (nums.iter().sum::<u32>() / nums.len() as u32) as f32
}

/// Capitalize every word.
///
/// For complex operation use https://github.com/wezm/titlecase
/// # Examples
///
/// Basic usage:
///
/// ``
/// assert_eq!("Foo Bar".to_string(), thirtyseconds::capitalize_every_word("foo bar".to_string()));
/// ```
pub fn capitalize_every_word(sentence: String) -> String {
    let words: Vec<&str> = sentence.split(' ').collect();
    let mut result: Vec<String> = Vec::new();

    for word in &words {
        result.push(format!(
            "{}{}",
            word.chars().nth(0).unwrap().to_uppercase().to_string(),
            &word[1..]
        ));
    }

    result.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_rgb() {
        assert_eq!(vec![255, 165, 1], hex_to_rgb(&"FFA501"));
        assert_eq!(vec![192, 192, 192], hex_to_rgb(&"C0C0C0"));
    }
    #[test]
    fn test_rgb_to_hex() {
        assert_eq!("FFA501", rgb_to_hex(255, 165, 1));
        assert_eq!(hex_to_rgb(&"C0C0C0"), vec![192, 192, 192]);
    }
    #[test]
    fn test_all_equal() {
        assert_eq!(true, all_equal(vec![2, 2, 2]));
        assert_eq!(false, all_equal(vec![2, 3, 2]));
    }
    #[test]
    fn test_all_unique() {
        assert_eq!(true, all_unique(vec![1, 2, 3]));
        assert_eq!(false, all_unique(vec![1, 1, 3]));
    }
    #[test]
    fn test_find_multiples() {
        assert_eq!(vec![5, 10, 15, 20, 25], find_multiples(5, 25));
        assert_eq!(vec![2, 4, 6, 8, 10], find_multiples(2, 10));
    }
    #[test]
    fn test_average() {
        assert_eq!(2.0, average(vec![1, 2, 3]));
        assert_eq!(4.0, average(vec![2, 4, 8]));
    }
    #[test]
    fn test_capitalize_every_word() {
        assert_eq!(
            "Hello World!".to_string(),
            capitalize_every_word("hello world!".to_string())
        );
        assert_eq!(
            "The Quick Brown Fox Jumps".to_string(),
            capitalize_every_word("The quick brown fox jumps".to_string())
        );
        assert_eq!("Foo".to_string(), capitalize_every_word("foo".to_string()));
    }
}
