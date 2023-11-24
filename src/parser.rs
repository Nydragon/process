pub enum DataError {
    Parsing,
}

pub trait Parser {
    fn parse() -> Result<Self, DataError>
    where
        Self: Sized;
}
