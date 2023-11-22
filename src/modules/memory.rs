use serde::{Deserialize, Serialize};

use crate::parser::Parser;

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
    fn parse() -> Option<Memory> {
        None
    }
}
