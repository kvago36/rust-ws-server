use serde::Serialize;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError, Serialize)]
pub enum MyError {
    #[error("Empty book title")]
    EmptyId,

    #[error("Empty book data")]
    EmptyBody,

    #[error("Invalid JSON")]
    ParsingError,

    #[error("Unknown action type")]
    UnknownAction,

    #[error("Book with this title already exists")]
    BookAlreadyExists,

    #[error("Can't find book with this title")]
    WrongId,
}