use process::data::Data;

#[test]
fn test_new() {
    let data = Data::new();

    assert!(data.cpu.is_some());
    assert!(data.memory.is_some());
}
