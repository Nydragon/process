#[macro_export]
macro_rules! timestamp {
    () => {
        std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    };
}
