use serde::{Deserialize, Serialize};

use crate::parser::Parser;

pub type Processes = Vec<Process>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Process {}

/// Implementing Parser for [Processes] instead of [Process].
impl Parser for Processes {
    fn parse() -> Option<Processes> {
        Some(vec![])
    }
}
