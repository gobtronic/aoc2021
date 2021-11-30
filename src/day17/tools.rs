use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;
use anyhow::Result;

pub fn read_values<T: FromStr>(source: &str) -> Result<Vec<T>> {
    let file = File::open(source)?;
    let lines = io::BufReader::new(file).lines();

    let mut values: Vec<T> = vec![];
    for line in lines {
        if let Ok(line) = line {
            match line.parse::<T>() {
                Ok(num) => values.push(num),
                _ => eprintln!("Error while parsing {}", line)
            }
        }
    }

    Ok(values)
}