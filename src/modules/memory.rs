use serde::{Deserialize, Serialize};

use crate::parser::Parser;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Memory {}

impl Parser for Memory {
    fn parse() -> Option<Memory> {
        None
    }
}
