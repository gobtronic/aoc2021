use std::cmp::Ordering;
use std::{collections::HashMap, fmt::Debug, hash::Hash, ops::RangeInclusive, str::FromStr};

mod tools;

fn main() {
    println!("Day result: {}", day::process(false));
    println!("Bonus result: {}", day::process(true));
}

pub struct Diagram {
    pub values: HashMap<Point, u16>,
}

impl Diagram {
    pub fn process(&mut self, lines: Vec<VentLine>, w_diagonals: bool) {
        for line in lines {
            match line.direction {
                Direction::Horizontal(range) => {
                    for x in range {
                        let point = Point {
                            x,
                            y: line.alignment,
                        };
                        if let Some(occurences) = self.values.get_mut(&point) {
                            *occurences += 1_u16;
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
                            *occurences += 1_u16;
                        } else {
                            self.values.insert(point, 1);
                        }
                    }
                }
                Direction::Diagonal(start, end) => {
                    if w_diagonals {
                        let diag_steps = (start.x as i64 - end.x as i64).abs().try_into().unwrap();

                        match (start.x.cmp(&end.x), start.y.cmp(&end.y)) {
                            (Ordering::Less, Ordering::Less) => {
                                for i in 0..=diag_steps {
                                    let point = Point {
                                        x: start.x + i,
                                        y: start.y + i,
                                    };
                                    if let Some(occurences) = self.values.get_mut(&point) {
                                        *occurences += 1_u16;
                                    } else {
                                        self.values.insert(point, 1);
                                    }
                                }
                            }
                            (Ordering::Less, Ordering::Greater) => {
                                for i in 0..=diag_steps {
                                    let point = Point {
                                        x: start.x + i,
                                        y: start.y - i,
                                    };
                                    if let Some(occurences) = self.values.get_mut(&point) {
                                        *occurences += 1_u16;
                                    } else {
                                        self.values.insert(point, 1);
                                    }
                                }
                            }
                            (Ordering::Greater, Ordering::Less) => {
                                for i in 0..=diag_steps {
                                    let point = Point {
                                        x: start.x - i,
                                        y: start.y + i,
                                    };
                                    if let Some(occurences) = self.values.get_mut(&point) {
                                        *occurences += 1_u16;
                                    } else {
                                        self.values.insert(point, 1);
                                    }
                                }
                            }
                            (Ordering::Greater, Ordering::Greater) => {
                                for i in 0..=diag_steps {
                                    let point = Point {
                                        x: start.x - i,
                                        y: start.y - i,
                                    };
                                    if let Some(occurences) = self.values.get_mut(&point) {
                                        *occurences += 1_u16;
                                    } else {
                                        self.values.insert(point, 1);
                                    }
                                }
                            }
                            _ => {}
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
    Diagonal(Point, Point),
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
        } else if (first_p.x as i64 - second_p.x as i64).abs()
            == (first_p.y as i64 - second_p.y as i64).abs()
        {
            // Alignment is irrelevant for Direction::Diagonal
            alignment = 0;
            direction = Direction::Diagonal(first_p, second_p);
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

    pub fn process(w_diagonals: bool) -> usize {
        if let Ok(values) = super::tools::read_values::<String>("src/day5/values.txt") {
            let mut diagram = Diagram::default();
            let vent_lines: Vec<VentLine> =
                values.iter().flat_map(|v| VentLine::from_str(v)).collect();
            diagram.process(vent_lines, w_diagonals);

            return diagram
                .values
                .iter()
                .filter(|(_, collisions)| collisions > &&1)
                .count();
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::{Point, Diagram, VentLine};

    #[test]
    fn mock_test() {
        let values = vec![
            "0,9 -> 5,9".to_string(),
            "8,0 -> 0,8".to_string(),
            "9,4 -> 3,4".to_string(),
            "2,2 -> 2,1".to_string(),
            "7,0 -> 7,4".to_string(),
            "6,4 -> 2,0".to_string(),
            "0,9 -> 2,9".to_string(),
            "3,4 -> 1,4".to_string(),
            "0,0 -> 8,8".to_string(),
            "5,5 -> 8,2".to_string()
        ];

        let mut diagram = Diagram::default();
        let vent_lines: Vec<VentLine> = values.iter().flat_map(|v| VentLine::from_str(v)).collect();
        diagram.process(vent_lines, false);
        let count =  diagram
            .values
            .iter()
            .filter(|(_, collisions)| collisions > &&1)
            .count();
        assert_eq!(count, 5);

        let mut diagram = Diagram::default();
        let vent_lines: Vec<VentLine> = values.iter().flat_map(|v| VentLine::from_str(v)).collect();
        diagram.process(vent_lines, true);
        let count =  diagram
            .values
            .iter()
            .filter(|(_, collisions)| collisions > &&1)
            .count();
        assert_eq!(count, 12);
    }
}
