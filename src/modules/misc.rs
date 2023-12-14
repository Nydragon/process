use std::{fs, process::Command};

use serde::{Deserialize, Serialize};

use crate::parser::Parser;

#[cfg(test)]
const UPTIME: &str = "./mock/uptime";
#[cfg(not(test))]
const UPTIME: &str = "/proc/uptime";

#[derive(Serialize, Deserialize, Debug)]
pub struct Misc {
    uptime: Option<f32>,
    clk_tck: Option<u8>,
}

impl Parser for Misc {
    fn parse() -> Result<Self, crate::parser::DataError>
    where
        Self: Sized,
    {
        Ok(Misc {
            uptime: Misc::get_uptime(),
            clk_tck: Misc::get_clktck(),
        })
    }
}

impl Misc {
    fn get_uptime() -> Option<f32> {
        fs::read_to_string(UPTIME).ok().and_then(|val| {
            val.split(' ')
                .next()
                .and_then(|val| val.parse::<f32>().ok())
        })
    }

    fn get_clktck() -> Option<u8> {
        Command::new("getconf")
            .arg("CLK_TCK")
            .output()
            .ok()
            .and_then(|r| {
                std::str::from_utf8(&r.stdout)
                    .ok()
                    .map(|s| String::from(s).replace('\n', ""))
            })
            .and_then(|res| res.parse::<u8>().ok())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_getclktck() {
        assert_ne!(Misc::get_clktck(), None);
    }

    #[test]
    fn test_get_uptime() {
        let up = Misc::get_uptime();

        assert_ne!(up, None);
    }
}
