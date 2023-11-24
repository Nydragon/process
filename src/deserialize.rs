use serde::de::{self, DeserializeOwned, DeserializeSeed, Error, MapAccess};
use serde::Deserialize;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum DeError {
    Eof,
    TrailingCharacters,
    ExpectedMapColon,
    ExpectedString,
    Default,
}

impl Display for DeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub struct Deserializer<'de> {
    input: &'de str,
}

impl<'de> Deserializer<'de> {
    pub fn from_str(input: &'de str) -> Self {
        Deserializer { input }
    }
}

pub fn from_str<'a, T: Deserialize<'a>>(s: &'a str) -> Result<T, DeError>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_str(s);

    match T::deserialize(&mut deserializer) {
        Ok(t) => {
            if deserializer.input.is_empty() {
                Ok(t)
            } else {
                Err(DeError::TrailingCharacters)
            }
        }
        Err(_) => Err(DeError::Default),
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

// Parsing functions
impl<'de> Deserializer<'de> {
    fn peek_char(&self) -> Result<char, DeError> {
        self.input.chars().next().ok_or(DeError::Eof)
    }

    fn peek_n_char(&self, n: usize) -> Result<char, DeError> {
        self.input.chars().nth(n).ok_or(DeError::Eof)
    }

    fn next_char(&mut self) -> Result<char, DeError> {
        let ch = self.peek_char()?;

        self.input = &self.input[ch.len_utf8()..];

        Ok(ch)
    }

    /// Parse a single string, only works for `values`
    fn parse_string(&mut self) -> Result<&'de str, DeError> {
        match self.input.find('\n') {
            Some(len) => {
                let s = &self.input[..len];
                self.input = &self.input[len + 1..];
                Ok(s)
            }
            None => Err(DeError::Eof),
        }
    }

    fn parse_key(&mut self) -> Result<&'de str, DeError> {
        if self.next_char()?.is_ascii_alphanumeric() {
            return Err(DeError::ExpectedString);
        }

        match self.input.find([':', ' ', '\t']) {
            Some(len) => {
                let s = &self.input[..len];
                self.input = &self.input[len + 1..];
                Ok(s)
            }
            None => Err(DeError::Eof),
        }
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
        if self.de.peek_n_char(0) == Ok('\n')
            && self.de.peek_n_char(1).err().unwrap() == DeError::Eof
        {
            return Ok(None);
        }

        // Deserialize a map key.
        seed.deserialize(&mut *self.de).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        // It doesn't make a difference whether the colon is parsed at the end
        // of `next_key_seed` or at the beginning of `next_value_seed`. In this
        // case the code is a bit simpler having it here.
        if self.de.next_char() != Ok(':') {
            return Err(Error::custom("Separator not present"));
        }
        // Deserialize a map value.
        seed.deserialize(&mut *self.de)
    }
}

// actual deserialization
impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = serde::de::value::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
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
        todo!()
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
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
        todo!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
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
        todo!()
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
        todo!()
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
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }
}

mod test {
    use super::*;

    #[derive(Deserialize, Debug, PartialEq, Eq)]
    struct Example {
        value1: u32,
        name: String,
    }

    #[test]
    fn test_deserialize() {
        let meminfo = "
            Value1:       32587776 kB
            Name:         Test";

        let parsed = from_str::<Example>(meminfo).unwrap();

        let comp = Example {
            value1: 32587776,
            name: String::from("Test"),
        };

        assert_eq!(parsed, comp)
    }
}
