use super::{modules::cpu::CPU, modules::memory::Memory};
use crate::{parser::Parser, timestamp};
use serde::{Deserialize, Serialize};

/// Holds all the system information
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub timestamp: u64,
    pub cpu: Option<CPU>,
    pub memory: Option<Memory>,
}

impl Data {
    pub fn new() -> Data {
        Data {
            timestamp: timestamp!(),
            cpu: CPU::parse().ok(),
            memory: Memory::parse().ok(),
        }
    }
}

impl Default for Data {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_collect() {
        Data::new();
    }
}
