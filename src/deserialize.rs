use core::panic;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;
use serde::de::{self, DeserializeOwned, DeserializeSeed, Error, MapAccess};
use serde::Deserialize;
use std::fmt::Display;
use std::ops::{AddAssign, MulAssign};
use std::str::FromStr;

#[derive(Parser)]
#[grammar = "proc.pest"]
struct ProcParser;

#[derive(Debug, PartialEq)]
pub enum DeError {
    Eof,
    TrailingCharacters,
    ExpectedMapColon,
    ExpectedString,
    Default,
    ExpectedInteger,
    InvalidBool,
}

impl Display for DeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub struct Deserializer<'de> {
    input: Pairs<'de, Rule>,
}

impl<'de> Deserializer<'de> {
    pub fn from_str(input: &'de str) -> Self {
        let proc = ProcParser::parse(Rule::file, input).unwrap_or_else(|e| panic!("{}", e));
        // println!("{:#?}", proc);
        Deserializer { input: proc }
    }
}

impl<'de> Deserializer<'de> {
    fn parse_unsigned<T>(&self, val: &str) -> Result<T, DeError>
    where
        T: AddAssign<T> + MulAssign<T> + From<u8> + FromStr,
    {
        let mut len = 0;

        for i in val.as_bytes() {
            if i.is_ascii_digit() {
                len = len + 1;
            }
        }

        match (&val[0..len]).parse() {
            Ok(v) => Ok(v),
            Err(_) => Err(DeError::ExpectedInteger),
        }
    }

    fn parse_bool(&self, val: &str) -> Result<bool, DeError> {
        match val {
            "yes" => Ok(true),
            "no" => Ok(false),
            _ => Err(DeError::InvalidBool),
        }
    }
}

pub fn from_str<'a, T: Deserialize<'a>>(s: &'a str) -> Result<T, DeError>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_str(s);

    match T::deserialize(&mut deserializer) {
        Ok(t) => {
            if deserializer.input.len() == 0 {
                Ok(t)
            } else {
                Err(DeError::TrailingCharacters)
            }
        }
        Err(e) => {
            println!("{}", e);
            Err(DeError::Default)
        }
    }
}

pub fn from_reader<R, T>(mut rdr: R) -> Result<T, DeError>
where
    R: std::io::Read,
    T: DeserializeOwned,
{
    let mut data = String::new();

    match rdr.read_to_string(&mut data) {
        Ok(_) => from_str(&data),
        Err(_) => Err(DeError::Default),
    }
}

/// [Format] is the representation of the file structure
struct Format<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> Format<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        Format { de }
    }
}

impl<'de, 'a> MapAccess<'de> for Format<'a, 'de> {
    type Error = serde::de::value::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        // Check if there are no more entries.
        if self.de.input.len() == 0 {
            return Ok(None);
        }

        // println!("{}", self.de.input);

        // Deserialize a map key.
        seed.deserialize(&mut *self.de).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let res = self.de.input.next();
        if res.is_some_and(|f| f.as_rule() != Rule::ass) {
            Err(Error::custom("Missing separator."))
        } else {
            // Deserialize a map

            seed.deserialize(&mut *self.de)
        }
    }
}

// actual deserialization
impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = serde::de::value::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.input.peek().unwrap().as_rule() {
            Rule::section => self.deserialize_map(visitor),
            Rule::key => self.deserialize_string(visitor),
            Rule::value => self.deserialize_string(visitor),
            _ => unimplemented!(),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let string = self.input.next().unwrap().as_str();

        let b = self.parse_bool(string);

        visitor.visit_bool(b.unwrap())
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let val = self.input.next().unwrap().as_str();

        let num = self.parse_unsigned::<u16>(val);

        visitor.visit_u16(num.unwrap())
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let val = self.input.next().unwrap().as_str();

        let num = self.parse_unsigned::<u32>(val);

        visitor.visit_u32(num.unwrap())
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let val = self.input.next().unwrap().as_str();

        let num = self.parse_unsigned::<u64>(val);

        visitor.visit_u64(num.unwrap())
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_borrowed_str(self.input.next().unwrap().as_str())
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        // Parse the opening brace of the map.
        // Give the visitor access to each entry of the map.
        visitor.visit_map(Format::new(self))
        // Parse the closing brace of the map.
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Deserialize, Debug, PartialEq, Eq)]
    struct Example {
        horse: bool,
        value1: u32,
        name: String,
    }

    type ExampleV = Vec<Example>;

    #[test]
    fn test_simple_deserialize() {
        let meminfo = "value1:       15 kB\nname:         Test\nhorse : yes\n";

        let parsed = from_str::<Example>(meminfo).unwrap();

        let comp = Example {
            horse: true,
            value1: 15,
            name: String::from("Test"),
        };

        assert_eq!(parsed, comp)
    }

    #[test]
    fn test_array_single_elem_deserialize() {
        let meminfo = "value1:       15 kB\nname:         Test\nhorse : yes\n";

        let parsed = from_str::<ExampleV>(meminfo).unwrap();

        let comp = [Example {
            horse: true,
            value1: 15,
            name: String::from("Test"),
        }];

        assert_eq!(parsed, comp)
    }

    #[test]
    fn test_array_multiple_elem_deserialize() {
        let meminfo = "value1:       15 kB\nname:         Test\nhorse : yes\n\nvalue1:       432 kB\nname:         Validation\nhorse : no\n";

        let parsed = from_str::<ExampleV>(meminfo).unwrap();

        let comp = [
            Example {
                horse: true,
                value1: 15,
                name: String::from("Test"),
            },
            Example {
                horse: false,
                value1: 432,
                name: String::from("Validation"),
            },
        ];

        assert_eq!(parsed, comp)
    }
}
