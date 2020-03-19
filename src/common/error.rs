use std::{io::Error, str::Utf8Error};
#[derive(Debug)]
pub enum SystemError {
    Internal(Box<Error>),
    Format(Box<Utf8Error>),
}

#[derive(Debug)]
pub enum ParserError {
    UnexpectedSymbol,
    List(Box<Vec<ParserError>>),
}

#[derive(Debug)]
pub enum MiniFnError {
    System(Box<SystemError>),
    LexicalError,
    ParsingError(Box<ParserError>),
    List(Box<Vec<MiniFnError>>),
}

impl MiniFnError {
    pub fn new_system_error(system: SystemError) -> Self {
        MiniFnError::System(Box::new(system))
    }

    pub fn new_lexical_error() -> Self {
        MiniFnError::LexicalError
    }

    pub fn new_parsing_error(parsing: ParserError) -> Self {
        MiniFnError::ParsingError(Box::new(parsing))
    }
}

impl From<Error> for MiniFnError {
    fn from(error: Error) -> Self {
        MiniFnError::System(Box::new(SystemError::Internal(Box::new(error))))
    }
}

impl From<Vec<ParserError>> for MiniFnError {
    fn from(errors: Vec<ParserError>) -> Self {
        MiniFnError::ParsingError(Box::new(ParserError::List(Box::new(errors))))
    }
}

impl From<Vec<MiniFnError>> for MiniFnError {
    fn from(errors: Vec<MiniFnError>) -> Self {
        MiniFnError::List(Box::new(errors))
    }
}

impl From<Utf8Error> for MiniFnError {
    fn from(error: Utf8Error) -> Self {
        MiniFnError::System(Box::new(SystemError::Format(Box::new(error))))
    }
}
