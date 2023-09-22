use std::{error::Error, fmt::Display};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum InvalidPriorityError {
    MissingParens,
    InvalidPriority,
}

impl Display for InvalidPriorityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingParens => write!(f, "Missing one or both parenthesis in todo priority"),
            Self::InvalidPriority => write!(f, "Invalid priority for todo priority"),
        }
    }
}

impl Error for InvalidPriorityError {}

#[derive(Clone, Debug)]
pub enum TodoParseError {
    BadDate,
    BadPriority,
}

impl Display for TodoParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for TodoParseError {}
