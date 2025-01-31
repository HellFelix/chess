use std::{error::Error, fmt::Display};

use chess_backend::ChessError;

#[derive(Debug)]
pub enum EngineError {
    DatabaseError,
    InputError,
}

impl Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Engine failure")
    }
}

impl Error for EngineError {}

impl From<ChessError> for EngineError {
    fn from(value: ChessError) -> Self {
        match value {
            ChessError::InputError => EngineError::InputError,
        }
    }
}
