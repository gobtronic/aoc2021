use std::{collections::HashMap, fmt::Debug, hash::Hash, ops::RangeInclusive, str::FromStr};

mod tools;

fn main() {
    println!("Day result: {}", day::process());
}

pub struct Diagram {
    pub values: HashMap<Point, u8>,
}

impl Diagram {
    pub fn process(&mut self, lines: Vec<VentLine>) {
        for line in lines {
            match line.direction {
                Direction::Horizontal(range) => {
                    for x in range {
                        let point = Point {
                            x,
                            y: line.alignment,
                        };
                        if let Some(occurences) = self.values.get_mut(&point) {
                            *occurences += 1_u8;
                        } else {
                            self.values.insert(point, 1);
                        }
                    }
                }
                Direction::Vertical(range) => {
                    for y in range {
                        let point = Point {
                            x: line.alignment,
                            y,
                        };
                        if let Some(occurences) = self.values.get_mut(&point) {
                            *occurences += 1_u8;
                        } else {
                            self.values.insert(point, 1);
                        }
                    }
                }
            }
        }
    }
}

impl Default for Diagram {
    fn default() -> Self {
        Diagram {
            values: HashMap::new(),
        }
    }
}

pub struct VentLine {
    pub alignment: u32,
    pub direction: Direction,
}

pub enum Direction {
    Horizontal(RangeInclusive<u32>),
    Vertical(RangeInclusive<u32>),
}

impl FromStr for VentLine {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<&str> = s.split(" -> ").collect();
        if values.len() != 2 {
            return Err(ParseError);
        }

        let first_p = Point::from_str(values[0]).unwrap();
        let second_p = Point::from_str(values[1]).unwrap();

        let alignment: u32;
        let direction: Direction;
        if first_p.x == second_p.x {
            alignment = first_p.x;
            if first_p.y < second_p.y {
                direction = Direction::Horizontal(first_p.y..=second_p.y);
            } else {
                direction = Direction::Horizontal(second_p.y..=first_p.y);
            }
        } else if first_p.y == second_p.y {
            alignment = first_p.y;
            if first_p.x < second_p.x {
                direction = Direction::Vertical(first_p.x..=second_p.x);
            } else {
                direction = Direction::Vertical(second_p.x..=first_p.x);
            }
        } else {
            return Err(ParseError);
        }

        Ok(VentLine {
            alignment,
            direction,
        })
    }
}

pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl FromStr for Point {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coordinates: Vec<u32> = s
            .split(',')
            .collect::<Vec<&str>>()
            .iter()
            .map(|v| v.parse::<u32>().unwrap())
            .collect();
        if coordinates.len() == 2 {
            return Ok(Point {
                x: coordinates[0],
                y: coordinates[1],
            });
        }

        Err(ParseError)
    }
}

impl Eq for Point {}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Hash for Point {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
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

    pub fn process() -> usize {
        if let Ok(values) = super::tools::read_values::<String>("src/day5/values.txt") {
            let mut diagram = Diagram::default();
            let vent_lines: Vec<VentLine> =
                values.iter().flat_map(|v| VentLine::from_str(v)).collect();
            diagram.process(vent_lines);

            return diagram
                .values
                .iter()
                .filter(|(_, collisions)| collisions > &&1)
                .count();
        }

        0
    }
}
