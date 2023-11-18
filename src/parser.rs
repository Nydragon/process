pub trait Parser {
    fn parse() -> Option<Self>
    where
        Self: Sized;
}
