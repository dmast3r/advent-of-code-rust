use std::fs;
use std::io::{self, BufRead};

pub fn read_input(day: u8) -> String {
    let path = format!("inputs/day{:02}.txt", day);
    fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Failed to read input file: {}", path))
}

pub fn read_input_lines(day: u8) -> io::Lines<io::BufReader<fs::File>> {
    let path = format!("inputs/day{:02}.txt", day);
    let file = fs::File::open(&path)
        .unwrap_or_else(|_| panic!("Failed to open input file: {}", path));
    io::BufReader::new(file).lines()
}
