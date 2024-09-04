use std::io::{self, BufRead};
use crate::utils;
use crate::utils::extract_num;

const MAX_RED: u16 = 12;
const MAX_GREEN: u16 = 13;
const MAX_BLUE: u16 = 14;

pub(crate) fn solve(path: &str, part_two: bool) -> io::Result<u16> {
    let reader = utils::open_file(path)?;
    let result = reader
        .lines()
        .filter_map(Result::ok)
        .filter_map(|line| {
            process(&line, part_two)
        })
        .sum::<u16>();
    return Ok(result);
}

fn process(line: &str, part_two: bool) -> Option<u16> {
    if part_two {
        return power_fewest(&line);
    }

    match is_valid(line) {
        Ok(value) => value,
        Err(value) => return value,
    }
}

fn power_fewest(line: &str) -> Option<u16> {
    let tokens: Vec<&str> = line.split(|c| c == ':' || c == ';' || c == ',').collect();

    let max_red = extract_max(&tokens, "red");
    let max_green = extract_max(&tokens, "green");
    let max_blue = extract_max(&tokens, "blue");

    max_red
        .zip(max_green)
        .zip(max_blue)
        .map(|((r, g), b)| r * g * b)
}

fn extract_max(tokens: &[&str], color: &str) -> Option<u16> {
    tokens.iter()
        .filter(|&t| (*t).contains(color))
        .filter_map(|&t| extract_num(t))
        .max()
}

fn is_valid(line: &str) -> Result<Option<u16>, Option<u16>> {
    let tokens: Vec<&str> = line.split(|c| c == ':' || c == ';' || c == ',').collect();

    let reds = is_possible(&tokens, "red", MAX_RED);
    let greens = is_possible(&tokens, "green", MAX_GREEN);
    let blues = is_possible(&tokens, "blue", MAX_BLUE);

    if (reds && greens && blues) {
        return Err(extract_num(&tokens[0]));
    }

    Ok(None)
}

fn is_possible(tokens: &[&str], color: &str, max: u16) -> bool {
    tokens.iter()
        .filter(|&t| (*t).contains(color))
        .filter_map(|&t| extract_num(t))
        .all(|n| n<= max)
}