use std::error;
use std::fmt::{Debug, Display, Formatter};
use std::fmt;

// 既存のResultをラップするResultを作りたい
pub type Result<T> = std::result::Result<T, dyn error::Error>;

// カスタムエラーを作りたい
#[derive(Debug)]
pub struct CustomError;

impl Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "custom error")
    }
}

// TODO: 何やってんの？
impl error::Error for CustomError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
