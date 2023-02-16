use std::{error::Error as StdError, fmt};

// create the error type that represents all errors possible in our program
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Fake(#[from] FakeError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Serde(#[from] serde_json::Error),

    #[error(transparent)]
    R2d2(#[from] r2d2::Error),

    #[error(transparent)]
    R2d2RustqliteSqlite(#[from] r2d2_sqlite::rusqlite::Error),
}

// we must manually implement serde::Serialize
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[derive(Debug, Clone)]
pub struct FakeError {
    pub message: String,
}

impl fmt::Display for FakeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "this is a fake error: {}", self.message)
    }
}

impl StdError for FakeError {}
