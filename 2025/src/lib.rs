use std::fs;

pub fn read_input(year: u16, day: u8) -> String {
    let path = format!("../inputs/{}/day{:02}.txt", year, day);
    fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Failed to read input file: {}", path))
}