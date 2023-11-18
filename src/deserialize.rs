use serde::de::value::Error;
use serde::de::{self};
use serde::Deserialize;
use std::fs;

pub enum Error {
    Eof,
    TrailingCharacters,
}

pub struct Deserializer<'de> {
    input: &'de str,
}

impl<'de> Deserializer<'de> {
    pub fn from_str(input: &'de str) -> Self {
        Deserializer { input }
    }
}

pub fn from_str<'a, T: Deserialize<'a>>(s: &'a str) -> Result<T, Error>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_str(s);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        Err(Error::TrailingCharacters)
    }
}

pub fn from_file<'a, T: Deserialize<'a>>(filename: &'a str) -> Result<T, Error>
where
    T: Deserialize<'a>,
{
    let content = fs::read(filename)?;
    let content = &String::from_utf8(content)?;

    return from_str(content);
}

// Parsing functions
impl<'de> Deserializer<'de> {
    fn peek_char(&self) -> Result<char, Error> {
        self.input.chars().next().ok_or(Error::Eof)
    }

    fn get_char(&self) -> Result<char, Error> {
        let ch = self.peek_char()?



        self.input = &self.input[ch.len_utf8().. ];

        Ok(ch)

    }
}

// actual deserialization
impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {}
