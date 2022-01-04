use std::error;
use std::fmt::{ Debug, Display };
use std::fmt;

// 既存のResultをラップするResultを作りたい
pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

// カスタムエラーを作りたい
#[derive(Debug)]
pub struct CustomError;

impl Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "custom error")
    }
}

impl error::Error for CustomError {}