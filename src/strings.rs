use crate::error::Error;

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
/// # use dryip::strings::hex_to_rgb;
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
/// # use dryip::strings::rgb_to_hex;
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
/// # use dryip::strings::capitalize_every_word;
/// assert_eq!("Foo Bar", capitalize_every_word("foo bar"));
/// ```
/// [RangeFull]: https://doc.rust-lang.org/std/ops/struct.RangeFull.html
pub fn capitalize_every_word(sentence: &str) -> String {
    sentence
        .split(' ')
        .map(|word| format!("{}{}", &word[..1].to_uppercase(), &word[1..]))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Converts a string to camelcase.
///
/// - Sanitize the input by removing leading and trailing whitespace, and replacing the required symbols.
/// - Reject any invalid input.
/// - Replace any - or _ with a space, using the `replace()`.
/// - Use `enumerate()` to check for the first word.
/// - Use [RangeFull] notation to get the first char uppercased and left the rest as is.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// # use dryip::strings::to_camelcase;
/// assert_eq!(Ok("fooBar".to_string()), to_camelcase("foo bar"));
/// ```
/// [RangeFull]: https://doc.rust-lang.org/std/ops/struct.RangeFull.html
pub fn to_camelcase(sentence: &str) -> Result<String, Error> {
    // sanitize the input first before the length check.
    // otherwise `-  ` will be valid input.
    let sentence_ = sentence
        .replace("-", " ")
        .replace("_", " ")
        .trim()
        .to_string();
    if !sentence_.is_ascii() {
        return Err(Error::NotAscii(sentence_));
    }
    if sentence_.len() < 2 {
        return Err(Error::InvalidWord(sentence_));
    }
    let result = sentence_
        .split(' ')
        .enumerate()
        .map(|(index, word)| {
            if index == 0 {
                word.to_lowercase()
            } else {
                format!("{}{}", &word[..1].to_uppercase(), &word[1..])
            }
        })
        .collect();
    Ok(result)
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
/// # use dryip::strings::is_anagram;
/// assert!(is_anagram("iceman", "cinema"));
/// ```
pub fn is_anagram(sentence1: &str, sentence2: &str) -> bool {
    let sentence1_ = sentence1.replace(" ", "").to_lowercase();
    let sentence2_ = sentence2.replace(" ", "").to_lowercase();

    if sentence1_.len() != sentence2_.len() {
        false
    } else {
        crate::sorted(&sentence1_) == crate::sorted(&sentence2_)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

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
        assert_eq!("Hello World!", capitalize_every_word("hello world!"));
        assert_eq!(
            "The Quick Brown Fox Jumps",
            capitalize_every_word("The quick brown fox jumps")
        );
        assert_eq!("Foo", capitalize_every_word("foo"));
    }
    #[test]
    fn test_to_camelcase() {
        //from proptest
        assert_eq!(
            Error::NotAscii("Σ".to_string()),
            to_camelcase("Σ").unwrap_err()
        );
        assert_eq!(
            Error::NotAscii("ପ".to_string()),
            to_camelcase("-ପ").unwrap_err()
        );
        assert_eq!(
            Error::InvalidWord("a".to_string()),
            to_camelcase("a ").unwrap_err()
        );
        assert_eq!(
            Error::InvalidWord("".to_string()),
            to_camelcase("-").unwrap_err()
        );

        assert_eq!(Ok("fooBar".to_string()), to_camelcase("foo bar"));
        assert_eq!(
            Ok("someDatabaseFieldName".to_string()),
            to_camelcase("some_database_field_name")
        );
        assert_eq!(
            Ok("someLabelThatNeedsToBeCamelized".to_string()),
            to_camelcase("Some label that needs to be camelized")
        );
        assert_eq!(
            Ok("someFooProperty".to_string()),
            to_camelcase("some-foo-property")
        );
        assert_eq!(
            Ok("someMixedStringWithSpacesUnderscoresAndHyphens".to_string()),
            to_camelcase("some-mixed_string with spaces_underscores-and-hyphens")
        )
    }
    #[test]
    fn test_is_anagram() {
        assert!(is_anagram("anagram", "Nag a ram"));
        assert!(is_anagram("iceman", "cinema"));
        assert!(!is_anagram("foo", "of"));
    }
    proptest! {
        #[test]
        fn strings_to_camelcase(s in "\\PC*") {
            println!("test input: {:?}", s);
            let _ = to_camelcase(&s);
        }
        #[test]
        fn strings_is_anagram(s in "\\PC*") {
            println!("test input: {:?}", s);
            let _ = is_anagram(&s, &s);
        }
    }
}
