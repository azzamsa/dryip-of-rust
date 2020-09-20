fn main() {
    println!("=== 30 Seconds of Rust ===");
    println!("hex_to_rgb: FFA501 to {:?}", hex_to_rgb("FFA501"));
}

/// convert HEX into RGB value
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// assert_eq!(vec![255, 165, 1], hex_to_rgb(&"FFA501"));
/// ```
fn hex_to_rgb(hex: &str) -> Vec<i64> {
    let to_base16 = |x| i64::from_str_radix(x, 16);
    // MAYBE can I collect as tuple directly?
    [0, 2, 4]
        .iter()
        .map(|&x| to_base16(&hex[x..x + 2]).unwrap())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_rgb() {
        assert_eq!(vec![255, 165, 1], hex_to_rgb(&"FFA501"));
    }
}
