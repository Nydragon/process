use super::{modules::cpu::CPUs, modules::memory::Memory};
use crate::{
    modules::{misc::Misc, process::Processes},
    parser::Parser,
    timestamp,
};
use serde::{Deserialize, Serialize};

/// Holds all the system information
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub timestamp: u64,
    pub cpu: Option<CPUs>,
    pub memory: Option<Memory>,
    pub processes: Option<Processes>,
    pub misc: Option<Misc>,
}

impl Data {
    pub fn new() -> Data {
        Data {
            timestamp: timestamp!(),
            cpu: CPUs::parse().ok(),
            memory: Memory::parse().ok(),
            processes: Processes::parse().ok(),
            misc: Misc::parse().ok(),
        }
    }
}

impl Default for Data {
    fn default() -> Self {
        Self::new()
    }
}
