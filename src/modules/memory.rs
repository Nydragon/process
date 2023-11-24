use crate::{
    deserialize::from_str,
    parser::{DataError, Parser},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Memory {
    total: u32,
    free: u32,
    available: u32,
    buffers: u32,
    cached: u32,
    swap_cached: u32,
    active: u32,
    inactive: u32,
    active_anon: u32,
    inactive_anon: u32,
    active_file: u32,
    inactive_file: u32,
    unevictable: u32,
    mlocked: u32,
    swap_total: u32,
    swap_free: u32,
    zswap: u32,
    zsapped: u32,
    dirty: u32,
    writeback: u32,
    anon_pages: u32,
    mapped: u32,
    shem: u32,
}

impl Parser for Memory {
    fn parse() -> Result<Memory, DataError> {
        let file = std::fs::read_to_string("/proc/meminfo");

        if let Ok(content) = file {
            match from_str(&content) {
                Ok(data) => Ok(data),
                Err(_) => Err(DataError::Parsing),
            }
        } else {
            return Err(DataError::FileNotFound);
        }
    }
}
