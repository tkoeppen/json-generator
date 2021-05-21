use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::error::GenErrorType::{Common, Parser, Generator, Sender};

#[derive(Debug)]
pub struct GenError {
    reason: String,
    tpe: GenErrorType,
}

#[derive(Debug)]
pub enum GenErrorType {
    Parser,
    Sender,
    Generator,
    Common,
}

impl GenError {
    pub fn new_with(reason: &str) -> Self {
        GenError { reason: reason.to_string(), tpe: Common }
    }
    pub fn new_with_in_parser(reason: &str) -> Self { GenError { reason: reason.to_string(), tpe: Parser } }
    pub fn new_with_in_sender(reason: &str) -> Self { GenError { reason: reason.to_string(), tpe: Sender } }
    pub fn new_with_in_generator(reason: &str) -> Self { GenError { reason: reason.to_string(), tpe: Generator } }
}

impl Error for GenError {}

impl From<std::io::Error> for GenError {
    fn from(e: std::io::Error) -> Self {
        GenError::new_with(format!("error from io, namely {}", e.to_string()).as_str())
    }
}

impl From<std::string::String> for GenError {
    fn from(e: std::string::String) -> Self {
        GenError::new_with(format!("error: {}", e.to_string()).as_str())
    }
}

impl From<nom::Err<nom::error::Error<&str>>> for GenError {
    fn from(e: nom::Err<nom::error::Error<&str>>) -> Self {
        GenError::new_with(e.to_string().as_str())
    }
}

impl From<serde_json::Error> for GenError {
    fn from(e: serde_json::Error) -> Self {
        GenError::new_with(e.to_string().as_str())
    }
}

impl Display for GenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "error while parsing a generator func, reason: {} and type: {:?}", self.reason, self.tpe)
    }
}