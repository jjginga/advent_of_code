use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub(crate) fn solve() -> io::Result<u16> {

    let path = Path::new("data/day01.txt");
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let result = reader
        .lines()
        .filter_map(Result::ok)
        .filter_map(|line| {
            let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
            format!("{}{}",digits.first().unwrap(), digits.last().unwrap()).parse::<u16>().ok()
            })
        .sum::<u16>();
    return Ok(result);
}