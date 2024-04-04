#![allow(non_snake_case)]

use std::fmt::{Formatter, write};

pub struct Error {
    ErrorCode: i16,
    ErrorMessage: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ErrorCode : {}, ErrorMessage : {}", self.ErrorCode, self.ErrorMessage)
    }
}



