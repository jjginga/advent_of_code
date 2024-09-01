use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

/**Main function to solve the problem the arguments are:
@convert: if true, the function will convert the words to digits
@path: path to the input file
*/
pub(crate) fn solve(path: &str, convert: bool) -> io::Result<u16> {

    let path = Path::new(path);
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    //processes the lines of the file, and converts them to the desired format
    //and then sums the values
    let result = reader
        .lines()
        .filter_map(Result::ok)
        .filter_map(|line| {
            extract_digits(&line, convert)
        })
        .sum::<u16>();
    return Ok(result);
}


/**Extracts the first and last digit of the line
@line: the line to be processed
@convert: if true, the function will convert the words to digits
*/
fn extract_digits(line: &str, convert: bool) -> Option<u16> {

    if !convert {
        //Part 1, this is simple, we just need to extract the digits from the line
        let digits: Vec<char> = line
                            .chars()
                            .filter(|c| c.is_digit(10))
                            .collect();
        return format!("{}{}",digits.first().unwrap(), digits.last().unwrap()).parse::<u16>().ok();
    }
    //Part 2, we need to convert the words to digits
    match_to_digits(&line)
}

/**Converts the words to digits
@line: the line to be processed
*/
fn match_to_digits(line: &str) -> Option<u16>{
    const NUMBERS: &[(&str, &str)] = &[
        ("one", "1"), ("two", "2"), ("three", "3"), ("four", "4"),
        ("five", "5"), ("six", "6"), ("seven", "7"), ("eight", "8"), ("nine", "9"),
        ("1","1"),("2","2"),("3","3"), ("4","4"), ("5","5"), ("6","6"), ("7", "7"),
        ("8", "8"), ("9", "9")
    ];

    let mut first: Option<&str> = None;
    let mut last: Option<&str> = None;

    let mut line_iter = line;

    //process the line from left to right, looking for matches to the numbers
    while !line_iter.is_empty() {
        let mut found = false;
        //checks if the line starts with a number
        for (key, value) in NUMBERS {
            if line_iter.starts_with(key) {
                //if the first number is found, we store it
                if first.is_none() {
                    first = Some(value);
                }

                last = Some(value);
                //move the iterator to the next position
                //we only move one position because of cases
                //like oneight
                line_iter = &line_iter[1..];
                found = true;
                break;
            }
        }
        //if no number is found, we move the iterator to the next position
        if !found {
            line_iter = &line_iter[1..];
        }
    }
   format!("{}{}", first.unwrap(), last.unwrap()).parse::<u16>().ok()
}