#![doc(
    html_logo_url = "https://raw.githubusercontent.com/azzamsa/dryip-of-rust/master/docs/drip.png
"
)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/azzamsa/dryip-of-rust/master/docs/drip.ico
"
)]

//! # Dryip of Rust 🦀
//!
//! Hello, and welcome to the Dryip of Rust 🦀 website! 👋
//!
//! - Each function categorized into appropriate module.
//! - Use the search bar to find the function.
//! - Click the `[SRC]` button at the top to see the complete source code.
//!

pub mod arrays;
pub mod dates;
pub mod maths;
pub mod strings;

pub fn sorted(word: &str) -> String {
    let mut chars_ = word.chars().collect::<Vec<char>>();
    // Rust currently doesn't have non-mutating sort function
    // https://github.com/rust-lang/rfcs/issues/2731
    // there is a handy tool https://docs.rs/itertools/0.10.1/itertools/fn.sorted.html
    // but currently, we avoid any external dependencies, as this project is only for
    // learning purpose
    chars_.sort_unstable();
    chars_.iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted() {
        assert_eq!("aaagmnr", sorted("anagram"));
        assert_eq!("aceimn", sorted("iceman"));
    }
}
