/// Convert HEX into RGB value.
///
/// - Convert string slice to integer using [from_str_radix].
/// - Iterate through fixed index for the hex input.
/// - Put the result into a vector.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// # use thirtyseconds::strings::hex_to_rgb;
/// assert_eq!(vec![255, 165, 1], hex_to_rgb("FFA501"));
/// ```
/// [from_str_radix]: https://doc.rust-lang.org/std/primitive.i64.html#method.from_str_radix
pub fn hex_to_rgb(hex: &str) -> Vec<i64> {
    let to_base16 = |x| i64::from_str_radix(x, 16);
    [0, 2, 4]
        .iter()
        .map(|&x| to_base16(&hex[x..x + 2]).unwrap())
        .collect::<Vec<_>>()
}

/// Convert RGB into HEX value.
///
/// Use [UpperHex] to format the string.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// # use thirtyseconds::strings::rgb_to_hex;
/// assert_eq!("FFA501", rgb_to_hex(255, 165, 1));
/// ```
/// [UpperHex]: https://doc.rust-lang.org/std/fmt/trait.UpperHex.html
pub fn rgb_to_hex(r: i32, g: i32, b: i32) -> String {
    format!("{:02X}{:02X}{:02X}", r, g, b)
}

/// Capitalize every word.
///
/// - Split the sentence by whitespace using `split()`.
/// - Iterate through each word using `map()`.
/// - Use [RangeFull] notation to get the first char uppercased and left the rest as is.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// # use thirtyseconds::strings::capitalize_every_word;
/// assert_eq!("Foo Bar".to_string(), capitalize_every_word("foo bar".to_string()));
/// ```
/// [RangeFull]: https://doc.rust-lang.org/std/ops/struct.RangeFull.html
pub fn capitalize_every_word(sentence: String) -> String {
    sentence
        .split(' ')
        .map(|word| format!("{}{}", &word[..1].to_uppercase(), &word[1..]))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Converts a string to camelcase.
///
/// - Replace any - or _ with a space, using the `replace()`.
/// - Use `enumerate()` to check for the first word.
/// - Use [RangeFull] notation to get the first char uppercased and left the rest as is.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// # use thirtyseconds::strings::to_camelcase;
/// assert_eq!("fooBar".to_string(), to_camelcase("foo bar".to_string()));
/// ```
/// [RangeFull]: https://doc.rust-lang.org/std/ops/struct.RangeFull.html
pub fn to_camelcase(sentence: String) -> String {
    sentence
        .replace("-", " ")
        .replace("_", " ")
        .split(' ')
        .enumerate()
        .map(|(index, word)| {
            if index == 0 {
                word.to_lowercase()
            } else {
                format!("{}{}", &word[..1].to_uppercase(), &word[1..])
            }
        })
        .collect()
}

/// Checks if a string is an anagram of another string (case-insensitive, ignores spaces, punctuation and special characters).
///
/// - Remove any whitespace using `replace()`.
/// - Lowercase all the string using `to_lowercase()`.
/// - Compare the lengths of the both, return False if they are not equal.
/// - If they have the same length, sort the string and compare.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// # use thirtyseconds::strings::is_anagram;
/// assert!(is_anagram("iceman".to_string(), "cinema".to_string()));
/// ```
pub fn is_anagram(sentence1: String, sentence2: String) -> bool {
    let sentence1_ = sentence1.replace(" ", "").to_lowercase();
    let sentence2_ = sentence2.replace(" ", "").to_lowercase();

    if sentence1_.len() != sentence2_.len() {
        false
    } else {
        crate::sorted(sentence1_) == crate::sorted(sentence2_)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_rgb() {
        assert_eq!(vec![255, 165, 1], hex_to_rgb("FFA501"));
        assert_eq!(vec![192, 192, 192], hex_to_rgb("C0C0C0"));
    }
    #[test]
    fn test_rgb_to_hex() {
        assert_eq!("FFA501", rgb_to_hex(255, 165, 1));
        assert_eq!(hex_to_rgb("C0C0C0"), vec![192, 192, 192]);
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
    #[test]
    fn test_to_camelcase() {
        assert_eq!("fooBar".to_string(), to_camelcase("foo bar".to_string()));
        assert_eq!(
            "someDatabaseFieldName".to_string(),
            to_camelcase("some_database_field_name".to_string())
        );
        assert_eq!(
            "someLabelThatNeedsToBeCamelized".to_string(),
            to_camelcase("Some label that needs to be camelized".to_string())
        );
        assert_eq!(
            "someFooProperty".to_string(),
            to_camelcase("some-foo-property".to_string())
        );
        assert_eq!(
            "someMixedStringWithSpacesUnderscoresAndHyphens".to_string(),
            to_camelcase("some-mixed_string with spaces_underscores-and-hyphens".to_string())
        )
    }
    #[test]
    fn test_is_anagram() {
        assert!(is_anagram("anagram".to_string(), "Nag a ram".to_string()));
        assert!(is_anagram("iceman".to_string(), "cinema".to_string()));
        assert!(!is_anagram("foo".to_string(), "of".to_string()));
    }
}
