use std::fs;
use std::io::{self, BufRead};

pub mod utils {
    use super::*;

    /// Reads the complete input file as a single String
    pub fn read_input(year: u16, day: u8) -> String {
        let path = format!("inputs/{}/day{:02}.txt", year, day);
        fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Failed to read input file: {}", path))
    }

    /// Returns an iterator over the lines of the input file
    pub fn read_input_lines(year: u16, day: u8) -> io::Lines<io::BufReader<fs::File>> {
        let path = format!("inputs/{}/day{:02}.txt", year, day);
        let file = fs::File::open(&path)
            .unwrap_or_else(|_| panic!("Failed to open input file: {}", path));
        io::BufReader::new(file).lines()
    }
}