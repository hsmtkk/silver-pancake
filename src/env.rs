pub fn mandatory_string(key: &str) -> String {
    let msg = format!("you must define {} environment variable", key);
    std::env::var(key).expect(&msg)
}
