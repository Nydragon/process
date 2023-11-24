use process::{modules::memory::Memory, parser::Parser};

#[test]
fn test_memory() {
    let data = Memory::parse();

    data.unwrap();
}
