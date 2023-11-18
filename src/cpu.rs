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
