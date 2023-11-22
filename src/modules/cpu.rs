use serde::{Deserialize, Serialize};

use crate::parser::Parser;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CPU {
    processor: u16,
    vendor_id: String,
    cpu_family: u16,
    model: usize,
    model_name: String,
    microcode: u32,
    cpu_mhz: f32,
    cache_size: usize,
    physical_id: u16,
    siblings: u16,
    core_id: u16,
    cpu_cores: u16,
    apicid: u16,
    initial_apicid: u16,
    fpu: bool,
    fpu_exception: bool,
    cpuid_level: u16,
    wp: bool,
    flags: Vec<String>,
    vmx_flags: Vec<String>,
    bugs: Vec<String>,
    bogomips: f32,
    clflush: u16,
    cache_alignment: u16,
    address_sizes: String,
    // power_management Option<>
}

impl Parser for CPU {
    fn parse() -> Option<CPU> {
        None
    }
}
