use std::{ops::RangeInclusive, str::FromStr, fmt::Debug};

mod tools;

fn main() {
    println!("Day result: {}", day::process());
}

pub struct Diagram {
    pub values: [[u8; 10]; 10]
}

impl Diagram {
    pub fn process(&mut self, line: VentLine) {
        
    }
}

impl Default for Diagram {
    fn default() -> Self {
        Diagram { values: [[0; 10]; 10] }    
    }
}

pub enum VentLine {
    Horizontal(RangeInclusive<u32>),
    Vertical(RangeInclusive<u32>)
}

impl FromStr for VentLine {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<&str> = s.split(" -> ").collect();
        if values.len() != 2 {
            return Err(ParseError)
        }

        let first_p = Point::from_str(values[0]).unwrap();
        let second_p = Point::from_str(values[1]).unwrap();

        if first_p.x == second_p.x {
            return Ok(VentLine::Vertical(first_p.y..=second_p.y))
        } else if first_p.y == second_p.y {
            return Ok(VentLine::Horizontal(first_p.x..=second_p.x))
        }

        Err(ParseError)
    }
}

pub struct Point {
    pub x: u32,
    pub y: u32
}

impl FromStr for Point {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coordinates: Vec<u32> = s.split(',').collect::<Vec<&str>>().iter().map(|v| v.parse::<u32>().unwrap()).collect();
        if coordinates.len() == 2 {
            return Ok(Point { x: coordinates[0], y: coordinates[1] })
        }

        return Err(ParseError)
    }
}

pub struct ParseError;

impl Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Something went wrong when parsing")
    }
}

mod day {
    use std::str::FromStr;

    use crate::{Diagram, VentLine};

    pub fn process() -> i32 {
        if let Ok(values) = super::tools::read_values::<String>("src/day5/values.txt") {
            let mut diagram = Diagram::default();
            //let vent_lines: Vec<VentLine> = values.iter().flat_map(|v| VentLine::from_str(v)).collect();
            //println!("{}", vent_lines.len());
        }

        0
    }
}