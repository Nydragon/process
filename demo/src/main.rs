use process::{
    modules::{cpu::CPUs, memory::Memory},
    parser::Parser,
    process::Processes,
};

fn main() {
    match Memory::parse() {
        Ok(mem) => println!("{}", serde_json::to_string_pretty(&mem).unwrap()),
        Err(_) => eprintln!("Unable to parse memory."),
    };

    match CPUs::parse() {
        Ok(cpu) => println!("{}", serde_json::to_string_pretty(&cpu).unwrap()),
        Err(_) => eprintln!("Unable to parse cpu."),
    };

    match Processes::parse() {
        Ok(pro) => println!("{}", serde_json::to_string_pretty(&pro).unwrap()),
        Err(_) => eprintln!("Unable to parse processes."),
    };
}
