use crate::parser::Parser;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, DirEntry},
};

#[cfg(test)]
const DEVICE_DIR: &str = "./mock/sysclassnet/";
#[cfg(not(test))]
const DEVICE_DIR: &str = "/sys/class/net/";

#[derive(Serialize, Deserialize, Debug)]
struct NetworkDevice {
    total_rx: Option<u64>,
    total_tx: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Network {
    devices: HashMap<String, NetworkDevice>,
}

impl Parser for Network {
    fn parse() -> Result<Self, crate::parser::DataError>
    where
        Self: Sized,
    {
        let mut hash: HashMap<String, NetworkDevice> = HashMap::new();

        fs::read_dir(DEVICE_DIR).ok().map(|dir| {
            dir.for_each(|dir| {
                let dir = dir.unwrap();
                let x = dir.file_name().to_str().map(|s| s.to_string());
                let data = NetworkDevice::parse(dir);

                if let Some(x) = x {
                    hash.insert(x, data)
                } else {
                    None
                };
            })
        });

        Ok(Network { devices: hash })
    }
}

impl NetworkDevice {
    fn parse(dir: DirEntry) -> NetworkDevice {
        let rxf = dir.path().join("statistics/rx_bytes");
        let txf = dir.path().join("statistics/tx_bytes");
        println!("{:?} ", fs::read_to_string(rxf.clone()));

        let rx = fs::read_to_string(rxf)
            .ok()
            .and_then(|s| s.trim().parse::<u64>().ok());
        let tx = fs::read_to_string(txf)
            .ok()
            .and_then(|s| s.trim().parse::<u64>().ok());

        NetworkDevice {
            total_rx: rx,
            total_tx: tx,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_network_parse() {
        let res = Network::parse().unwrap();

        assert_eq!(res.devices.len(), 1);
    }

    #[test]
    fn test_network_device_parse() {
        let dir = fs::read_dir(DEVICE_DIR)
            .expect("1")
            .nth(0)
            .expect("2")
            .expect("3");

        let res = NetworkDevice::parse(dir);

        assert_ne!(res.total_rx, None);
        assert_ne!(res.total_tx, None);
    }
}
