/// all possible errors returned by the app.
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
    #[error("invalid word: `{0}`,  expected at least 2 char")]
    InvalidWord(String),

    #[error("input must be lowercase but was `{0}`")]
    NotAscii(String),
}
