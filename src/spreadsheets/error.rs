use thiserror::Error;

#[derive(Error, Debug)]
pub enum FilesystemError {
    #[error("file not found: {path}")]
    FileNotFound { path: String },
}

#[derive(Error, Debug)]
pub enum SpreadsheetError {
    #[error("too many columns in line {line}. Expected {expected} but found {found}")]
    TooManyColumns {
        line: usize,
        expected: usize,
        found: usize,
    },

    #[error("not enough columns in line {line}. Expected {expected} but found {found}")]
    NotEnoughColumns {
        line: usize,
        expected: usize,
        found: usize,
    },
}
