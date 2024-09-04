use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub(crate) fn open_file(path: &str) -> Result<BufReader<File>, std::io::Error> {
    let path = Path::new(path);
    let file = File::open(&path)?;
    Ok(BufReader::new(file))
}

pub(crate) fn extract_num(s: &str) -> Option<u16> {
    s.chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<u16>()
        .ok()
}