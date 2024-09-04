mod day01;
mod utils;
mod day02;

fn main() {
    println!("Advent of Code 2023");
    println!("Solution for day 1 - part 1: {}", day01::solve("data/day01.txt", false).unwrap());
    println!("Solution for day 1 - part 2: {}", day01::solve("data/day01.txt", true).unwrap());
    println!("Solution for day 2 - part 1: {}", day02::solve("data/day02.txt", false).unwrap());
    println!("Solution for day 2 - part 2: {}", day02::solve("data/day02.txt", true).unwrap());
}
