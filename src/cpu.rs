use serde::{Deserialize, Serialize};

use crate::parser::Parser;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CPU {}

impl Parser for CPU {
    fn parse() -> Option<CPU> {
        None
    }
}

impl<'de> Deserialize<'de> for CPU {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
    }
}
