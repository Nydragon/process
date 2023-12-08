use crate::{
    deserialize::from_str,
    parser::{DataError, Parser},
};
use serde::{Deserialize, Serialize};

#[cfg(not(test))]
const CPUINFO: &str = "/proc/cpuinfo";
#[cfg(test)]
const CPUINFO: &str = "./mock/cpuinfo";

#[derive(Serialize, Deserialize, Debug)]
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

impl CPU {
    fn parse_sec(e: &str) -> CPU {
        let mut e = String::from(e);
        if !e.ends_with('\n') {
            e.push('\n');
        }

        from_str::<CPU>(&e).unwrap()
    }
}

impl Parser for CPUs {
    fn parse() -> Result<CPUs, DataError> {
        let file = std::fs::read_to_string(CPUINFO).unwrap();

        Ok(file.split_inclusive("\n\n").map(CPU::parse_sec).collect())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        CPUs::parse().unwrap();
    }
}
