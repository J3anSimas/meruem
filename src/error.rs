use thiserror::Error;
#[derive(Error, Debug)]
pub enum Error {
    #[error("You tried position {idx} but the last index possible is 4")]
    OutOfBounds { idx: usize },
    #[error("The path {path} that you passed is not valid")]
    InvalidPath { path: String },
}
