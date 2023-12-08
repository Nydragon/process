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
    /// The process ID.
    pid: u32,
    /// The filename of the executable, in parentheses.
    /// Strings longer than TASK_COMM_LEN (16) characters
    /// (including the terminating null byte) are silently
    /// truncated.  This is visible whether or not the
    /// executable is swapped out.
    command: String,
    /// The current state, refer to [State] for more information.
    state: State,
    /// Parent Process ID
    ppid: u32,
    /// The process group ID of the process.
    pgrp: u32,
    /// The session ID of the process.
    session: u32,
    ///  The controlling terminal of the process. (The minor device number is contained in the combination of bits 31 to 20 and 7 to 0; the major device number is in bits 15 to 8.)
    tty_nr: u32,
    /// The ID of the foreground process group of the controlling terminal of the process.
    tpgid: i32,
    /// The kernel flags word of the process.  For bit
    /// meanings, see the PF_* defines in the Linux kernel
    /// source file include/linux/sched.h.  Details depend
    /// on the kernel version.
    ///
    /// The format for this field was %lu before Linux 2.6.
    flags: u32,
    /// The number of minor faults the process has made
    /// which have not required loading a memory page from
    /// disk.
    minflit: u32,
    ///  The number of minor faults that the process's waited-for children have made.
    cminflit: u32,
    majflt: u32,
    cmajflt: u32,
    utime: u32,
    stime: u32,
    cutime: u32,
    cstime: u32,
    priority: i32,
    nice: i32,
    num_threads: i32,
    itrealvalue: i32,
    starttime: u64,
    vsize: u64,
    rss: i32,
    rsslim: u64,
    startcode: u64,
    encode: u64,
    startstack: u64,
    kstkep: u64,
    kstkeip: u64,
    signal: u64,
    blocked: u64,
    sigignore: u64,
    sigcatch: u64,
    wchan: u64,
    nswap: u64,
    cnswap: u64,
    exit_signal: i32,
    processor: i32,
    rt_priotiy: u32,
    policy: u32,
    delayacct_blkio_ticks: u64,
    guest_time: u64,
    cguest_time: u64,
    start_data: u64,
    end_data: u64,
    start_brk: u64,
    arg_start: u64,
    arg_end: u64,
    env_start: u64,
    env_end: u64,
    exit: i32,
}

impl Process {
    fn new(stat: &str) -> Result<Process, Box<dyn std::error::Error>> {
        let mut stats = StatParser::parse(Rule::line, stat).expect("Hello");
        println!("{} {}", stat, stats.len());
        Ok(Process {
            pid: stats.next().unwrap().as_str().parse().unwrap(),
            command: stats.next().unwrap().as_str().parse().unwrap(),
            state: State::from_str(stats.next().unwrap().as_str()).unwrap(),
            ppid: stats.next().unwrap().as_str().parse().unwrap(),
            pgrp: stats.next().unwrap().as_str().parse().unwrap(),
            session: stats.next().unwrap().as_str().parse().unwrap(),
            tty_nr: stats.next().unwrap().as_str().parse().unwrap(),
            tpgid: stats.next().unwrap().as_str().parse().unwrap(),
            flags: stats.next().unwrap().as_str().parse().unwrap(),
            minflit: stats.next().unwrap().as_str().parse().unwrap(),
            cminflit: stats.next().unwrap().as_str().parse().unwrap(),
            majflt: stats.next().unwrap().as_str().parse().unwrap(),
            cmajflt: stats.next().unwrap().as_str().parse().unwrap(),
            utime: stats.next().unwrap().as_str().parse().unwrap(),
            stime: stats.next().unwrap().as_str().parse().unwrap(),
            cutime: stats.next().unwrap().as_str().parse().unwrap(),
            cstime: stats.next().unwrap().as_str().parse().unwrap(),
            priority: stats.next().unwrap().as_str().parse().unwrap(),
            nice: stats.next().unwrap().as_str().parse().unwrap(),
            num_threads: stats.next().unwrap().as_str().parse().unwrap(),
            itrealvalue: stats.next().unwrap().as_str().parse().unwrap(),
            starttime: stats.next().unwrap().as_str().parse().unwrap(),
            vsize: stats.next().unwrap().as_str().parse().unwrap(),
            rss: stats.next().unwrap().as_str().parse().unwrap(),
            rsslim: stats.next().unwrap().as_str().parse().unwrap(),
            startcode: stats.next().unwrap().as_str().parse().unwrap(),
            encode: stats.next().unwrap().as_str().parse().unwrap(),
            startstack: stats.next().unwrap().as_str().parse().unwrap(),
            kstkep: stats.next().unwrap().as_str().parse().unwrap(),
            kstkeip: stats.next().unwrap().as_str().parse().unwrap(),
            signal: stats.next().unwrap().as_str().parse().unwrap(),
            blocked: stats.next().unwrap().as_str().parse().unwrap(),
            sigignore: stats.next().unwrap().as_str().parse().unwrap(),
            sigcatch: stats.next().unwrap().as_str().parse().unwrap(),
            wchan: stats.next().unwrap().as_str().parse().unwrap(),
            nswap: stats.next().unwrap().as_str().parse().unwrap(),
            cnswap: stats.next().unwrap().as_str().parse().unwrap(),
            exit_signal: stats.next().unwrap().as_str().parse().unwrap(),
            processor: stats.next().unwrap().as_str().parse().unwrap(),
            rt_priotiy: stats.next().unwrap().as_str().parse().unwrap(),
            policy: stats.next().unwrap().as_str().parse().unwrap(),
            delayacct_blkio_ticks: stats.next().unwrap().as_str().parse().unwrap(),
            guest_time: stats.next().unwrap().as_str().parse().unwrap(),
            cguest_time: stats.next().unwrap().as_str().parse().unwrap(),
            start_data: stats.next().unwrap().as_str().parse().unwrap(),
            end_data: stats.next().unwrap().as_str().parse().unwrap(),
            start_brk: stats.next().unwrap().as_str().parse().unwrap(),
            arg_start: stats.next().unwrap().as_str().parse().unwrap(),
            arg_end: stats.next().unwrap().as_str().parse().unwrap(),
            env_start: stats.next().unwrap().as_str().parse().unwrap(),
            env_end: stats.next().unwrap().as_str().parse().unwrap(),
            exit: stats.next().unwrap().as_str().parse().unwrap(),
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        Process::new("1 (systemd) S 0 1 1 0 -1 4194560 643700 109464643 189 69440 268 818 7489706 1155578 20 0 1 0 12 23293952 3585 18446744073709551615 1 1 0 0 0 0 671173123 4096 1260 0 0 0 17 5 0 0 0 0 0 0 0 0 0 0 0 0 0\n").expect("");
    }

    #[test]
    fn test_parse_2() {
        Process::new("252201 (kworker/u33:3+i915_flip) D 2 0 0 0 -1 69238880 0 0 0 0 0 282 0 0 0 -20 1 0 16881972 0 0 18446744073709551615 0 0 0 0 0 0 0 2147483647 0 0 0 0 17 8 0 0 0 0 0 0 0 0 0 0 0 0 0\n").expect("");
    }
}
