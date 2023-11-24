#[derive(Debug)]
pub enum DataError {
    Parsing,
    FileNotFound,
}

pub trait Parser {
    fn parse() -> Result<Self, DataError>
    where
        Self: Sized;
}
