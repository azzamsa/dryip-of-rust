use std::collections::HashSet;

/// convert HEX into RGB value
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

/// convert RGB into HEX value
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

/// check if all value in given list are equal
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// assert_eq!(true, thirtyseconds::all_equal(vec![2, 2, 2]));
/// ```
pub fn all_equal(lst: Vec<u32>) -> bool {
    // FIX why the python version use lst[1:] == lst[:-1]?
    &lst[..] == &lst[..]
}

/// check if all value in given list are unique
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// assert_eq!(true, thirtyseconds::all_equal(vec![1, 2, 3]));
/// ```
pub fn all_unique(lst: Vec<u32>) -> bool {
    lst.len() == lst.into_iter().collect::<HashSet<u32>>().len()
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
        assert_eq!(true, all_equal(vec![2, 3, 2]));
    }
    #[test]
    fn test_all_unique() {
        assert_eq!(true, all_unique(vec![1, 2, 3]));
        assert_eq!(false, all_unique(vec![1, 1, 3]));
    }
}
