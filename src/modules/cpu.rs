use crate::{
    deserialize::from_str,
    parser::{DataError, Parser},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CPU {
    processor: u16,
    vendor_id: String,
    #[serde(rename(deserialize = "cpu family"))]
    cpu_family: u16,
    model: usize,
    #[serde(rename(deserialize = "model name"))]
    model_name: String,
    microcode: String,
    #[serde(rename(deserialize = "cpu MHz"))]
    cpu_mhz: String, // IMPLEMENT FLOAT PARSING
    #[serde(rename(deserialize = "cache size"))]
    cache_size: usize,
    #[serde(rename(deserialize = "physical id"))]
    physical_id: u16,
    siblings: u16,
    #[serde(rename(deserialize = "core id"))]
    core_id: u16,
    #[serde(rename(deserialize = "cpu cores"))]
    cpu_cores: u16,
    apicid: u16,
    #[serde(rename(deserialize = "initial apicid"))]
    initial_apicid: u16,
    fpu: bool,
    fpu_exception: bool,
    #[serde(rename(deserialize = "cpuid level"))]
    cpuid_level: u16,
    wp: bool,
    flags: String, //Vec<String>,
    #[serde(rename(deserialize = "vmx flags"))]
    vmx_flags: String, //Vec<String>,
    bugs: String,  // Vec<String>,
    bogomips: String,
    #[serde(rename(deserialize = "clflush size"))]
    clflush_size: u16,
    cache_alignment: u16,
    #[serde(rename(deserialize = "address sizes"))]
    address_sizes: String,
    power_management: Option<u8>,
}

pub type CPUs = Vec<CPU>;

impl Parser for CPUs {
    fn parse() -> Result<CPUs, DataError> {
        let file = std::fs::read_to_string("/proc/cpuinfo");

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
        let meminfo = fs::read_to_string("./mock/cpuinfo").unwrap();

        from_str::<CPU>(&meminfo).unwrap();
    }
}
