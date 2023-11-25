use serde::{Deserialize, Serialize};

use crate::parser::{DataError, Parser};

pub type Processes = Vec<Process>;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct Process {
    /// Process ID
    pid: u32,
    /// Parent Process ID
    ppid: u32,
    /// User
    user: String,
    priority: u8,
    niceness: u8,
    virt: u32,
    res: u32,
    shr: u32,
    s: State,
    cpu_abs: u32,
    mem_abs: u32,
    time: u32,
    command: String,
}

/// Implementing Parser for [Processes] instead of [Process].
impl Parser for Processes {
    fn parse() -> Result<Processes, DataError> {
        unimplemented!()
    }
}
