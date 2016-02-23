pub fn to_bytes(s: &str) -> u8 {
    match s {
        "AAAA"=> 0,
        _ => 1,
    }
}
