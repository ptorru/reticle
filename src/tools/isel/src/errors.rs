use asm::errors::Error as AsmError;
use ir::errors::Error as IRError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    IR(IRError),
    Asm(AsmError),
    ISel(String),
}

impl Error {
    pub fn new_isel_error(msg: &str) -> Self {
        Error::ISel(msg.to_string())
    }
}

impl From<IRError> for Error {
    fn from(e: IRError) -> Self {
        Error::IR(e)
    }
}

impl From<AsmError> for Error {
    fn from(e: AsmError) -> Self {
        Error::Asm(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IR(msg) => write!(f, "{}", msg),
            Error::Asm(msg) => write!(f, "{}", msg),
            Error::ISel(msg) => write!(f, "{}", msg),
        }
    }
}
