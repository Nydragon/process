use crate::{
    deserialize::from_str,
    parser::{DataError, Parser as PParser},
};
use pest::Parser;
use serde::{Deserialize, Serialize};
use std::{fs, str::FromStr};

pub type Processes = Vec<Process>;

#[derive(pest_derive::Parser)]
#[grammar = "stat.pest"]
struct StatParser;

#[derive(Serialize, Deserialize, Debug)]
enum State {
    /// Sleeping
    S = 1,
    /// Idle
    I = 2,
    /// Running
    R = 3,
    ///  Disk Sleep
    D = 4,
    /// Zombie (waiting for parent to read its exit status)
    Z = 5,
    /// traced or suspended (e.g by SIGTSTP)
    T = 6,
    /// Paging
    W = 7,
}

impl FromStr for State {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().nth(0).unwrap() {
            'S' => Ok(State::S),
            'I' => Ok(State::I),
            'R' => Ok(State::R),
            'D' => Ok(State::D),
            'Z' => Ok(State::Z),
            'T' => Ok(State::T),
            'W' => Ok(State::W),
            s => Err(format!("Bad State: {}", s)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Process {
    /// Process ID
    pid: u32,
    command: String,
    // /// The current state, refer to [State] for more information.
    // state: State,
    // /// Parent Process ID
    // ppid: u32,
    // /// User
    // user: String,
    // priority: u8,
    // niceness: u8,
    // virt: u32,
    // res: u32,
    // shr: u32,
    // cpu_abs: u32,
    // mem_abs: u32,
    // time: u32,
    // allocated_fds: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct ProcessRaw {
    #[serde(rename(deserialize = "Name"))]
    command: String,
    #[serde(rename(deserialize = "Pid"))]
    pid: u32,
    #[serde(rename(deserialize = "State"))]
    state: String,
    #[serde(rename(deserialize = "PPid"))]
    ppid: u32,
    // #[serde(rename(deserialize = "FDSize"))]
    // allocated_fds: u32,
}

impl Process {
    fn new(stat: &str) -> Result<Process, Box<dyn std::error::Error>> {
        let mut stats = StatParser::parse(Rule::line, stat).expect("Hello");

        Ok(Process {
            pid: stats.next().unwrap().as_str().parse().unwrap(),
            command: stats.next().unwrap().as_str().parse().unwrap(),
            // state: State::from_str(&proc.state).unwrap(),
            // ppid: proc.ppid,
            // user: todo!(),
            // priority: todo!(),
            // niceness: todo!(),
            // virt: todo!(),
            // res: todo!(),
            // shr: todo!(),
            // cpu_abs: todo!(),
            // mem_abs: todo!(),
            // time: todo!(),
        })
    }
}

/// Implementing Parser for [Processes] instead of [Process].
impl PParser for Processes {
    fn parse() -> Result<Processes, DataError> {
        Ok(fs::read_dir("/proc/")
            .expect("")
            .filter_map(|entry| {
                let entry = entry.expect("");
                let path = entry.path();

                let folder_name = path.file_name().unwrap();

                let x = folder_name
                    .to_str()
                    .unwrap()
                    .chars()
                    .all(|c| c.is_digit(10));

                if x {
                    let str = &fs::read_to_string(entry.path().join("stat")).expect("msg");

                    Some(Process::new(str).unwrap())
                } else {
                    None
                }
            })
            .collect())
    }
}
