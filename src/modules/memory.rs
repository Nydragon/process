use crate::{
    deserialize::from_str,
    parser::{DataError, Parser},
};
use serde::{Deserialize, Serialize};

/// Rust representation of the contents of `/proc/meminfo``
#[derive(Serialize, Deserialize, Debug)]
pub struct Memory {
    #[serde(rename(deserialize = "MemTotal"))]
    total: u32,
    #[serde(rename(deserialize = "MemFree"))]
    free: u32,
    #[serde(rename(deserialize = "MemAvailable"))]
    available: u32,
    #[serde(rename(deserialize = "Buffers"))]
    buffers: u32,
    #[serde(rename(deserialize = "Cached"))]
    cached: u32,
    #[serde(rename(deserialize = "SwapCached"))]
    swap_cached: u32,
    #[serde(rename(deserialize = "Active"))]
    active: u32,
    #[serde(rename(deserialize = "Inactive"))]
    inactive: u32,
    #[serde(rename(deserialize = "Active(anon)"))]
    active_anon: u32,
    #[serde(rename(deserialize = "Inactive(anon)"))]
    inactive_anon: u32,
    #[serde(rename(deserialize = "Active(file)"))]
    active_file: u32,
    #[serde(rename(deserialize = "Inactive(file)"))]
    inactive_file: u32,
    #[serde(rename(deserialize = "Unevictable"))]
    unevictable: u32,
    #[serde(rename(deserialize = "Mlocked"))]
    mlocked: u32,
    #[serde(rename(deserialize = "SwapTotal"))]
    swap_total: u32,
    #[serde(rename(deserialize = "SwapFree"))]
    swap_free: u32,
    #[serde(rename(deserialize = "Zswap"))]
    zswap: u32,
    #[serde(rename(deserialize = "Zswapped"))]
    zsapped: u32,
    #[serde(rename(deserialize = "Dirty"))]
    dirty: u32,
    #[serde(rename(deserialize = "Writeback"))]
    writeback: u32,
    #[serde(rename(deserialize = "AnonPages"))]
    anon_pages: u32,
    #[serde(rename(deserialize = "Mapped"))]
    mapped: u32,
    #[serde(rename(deserialize = "Shmem"))]
    shmem: u32,
    // Missing more data
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

#[cfg(test)]
mod test {
    use std::fs;

    use super::*;

    #[test]
    fn test_parse() {
        let meminfo = fs::read_to_string("./mock/meminfo").unwrap();

        from_str::<Memory>(&meminfo).unwrap();
    }
}
