use std::convert::From;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct RuntimeError(String);

impl fmt::Display for RuntimeError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "RuntimeError -> {}", &self.0)
    }
}

impl Error for RuntimeError {}

pub struct EmptyStackError<'a> {
    message: &'a str,
}

impl<'a> EmptyStackError<'a> {
    pub fn new(message: &'a str) -> Self {
        EmptyStackError { message }
    }
}

impl<'a> fmt::Display for EmptyStackError<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "EmpyStackError: There is no obligatory parameter in the stack. LINE => {}",
            self.message
        )
    }
}

impl<'a> From<EmptyStackError<'a>> for RuntimeError {
    fn from(error: EmptyStackError) -> Self {
        RuntimeError(error.to_string())
    }
}

pub struct IntegerOverflowError<'a> {
    message: &'a str,
}

impl<'a> IntegerOverflowError<'a> {
    pub fn new(message: &'a str) -> Self {
        IntegerOverflowError { message }
    }
}

impl<'a> fmt::Display for IntegerOverflowError<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "IntegerOverflowError: Result of an integer operation does not fit within the allocated memory space. LINE => {}", self.message)
    }
}

impl<'a> From<IntegerOverflowError<'a>> for RuntimeError {
    fn from(error: IntegerOverflowError) -> Self {
        RuntimeError(error.to_string())
    }
}
pub struct UnknownVariableLoadingError<'a> {
    message: &'a str,
}

impl<'a> UnknownVariableLoadingError<'a> {
    pub fn new(message: &'a str) -> Self {
        UnknownVariableLoadingError { message }
    }
}

impl<'a> fmt::Display for UnknownVariableLoadingError<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "UnkonwnVariableLoadingError: There is no such variable in the context. LINE => {}",
            self.message
        )
    }
}

impl<'a> From<UnknownVariableLoadingError<'a>> for RuntimeError {
    fn from(error: UnknownVariableLoadingError) -> Self {
        RuntimeError(error.to_string())
    }
}

pub struct NoReturnCommandError;

impl fmt::Display for NoReturnCommandError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "NoReturnCommandError: There is no RETURN command in the programm. RETURN command is obligatory.")
    }
}

impl From<NoReturnCommandError> for RuntimeError {
    fn from(error: NoReturnCommandError) -> Self {
        RuntimeError(error.to_string())
    }
}

pub struct LoopInitError<'a> {
    message: &'a str,
}

impl<'a> LoopInitError<'a> {
    pub fn new(message: &'a str) -> Self {
        LoopInitError { message }
    }
}

impl<'a> fmt::Display for LoopInitError<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "LoopInitError: Loop was not initialized approprietly LINE => {}",
            self.message
        )
    }
}

impl<'a> From<LoopInitError<'a>> for RuntimeError {
    fn from(error: LoopInitError) -> Self {
        RuntimeError(error.to_string())
    }
}

pub struct NoLoopInstanceError;

impl fmt::Display for NoLoopInstanceError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "NoLoopInstanceError: There is no implementaion for loop but LOOP instruction been used")
    }
}

impl From<NoLoopInstanceError> for RuntimeError {
    fn from(error: NoLoopInstanceError) -> Self {
        RuntimeError(error.to_string())
    }
}

pub struct NestedLoopsError<'a> {
    message: &'a str,
}

impl<'a> NestedLoopsError<'a> {
    pub fn new(message: &'a str) -> Self {
        NestedLoopsError { message }
    }
}

impl<'a> fmt::Display for NestedLoopsError<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "NestedLoopsError: Nested loops are forbiden LINE => {}",
            self.message
        )
    }
}

impl<'a> From<NestedLoopsError<'a>> for RuntimeError {
    fn from(error: NestedLoopsError) -> Self {
        RuntimeError(error.to_string())
    }
}

pub struct NoCodeError;

impl fmt::Display for NoCodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "NoCodeError: There is no code to interpritate.")
    }
}

impl From<NoCodeError> for RuntimeError {
    fn from(error: NoCodeError) -> Self {
        RuntimeError(error.to_string())
    }
}

impl From<NoCodeError> for ParserError {
    fn from(error: NoCodeError) -> Self {
        ParserError(error.to_string())
    }
}

#[derive(Debug)]
pub struct ParserError(String);

impl fmt::Display for ParserError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "ParseError -> {}", &self.0)
    }
}

impl Error for ParserError {}

pub struct UnknownCommandError<'a> {
    message: &'a str,
}

impl<'a> UnknownCommandError<'a> {
    pub fn new(message: &'a str) -> Self {
        UnknownCommandError { message }
    }
}

impl<'a> fmt::Display for UnknownCommandError<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "UknownCommandError: The command is unkonwn or syntax is wrong LINE => {}",
            self.message
        )
    }
}

impl<'a> From<UnknownCommandError<'a>> for ParserError {
    fn from(error: UnknownCommandError) -> Self {
        ParserError(error.to_string())
    }
}
