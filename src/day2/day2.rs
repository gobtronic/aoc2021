mod tools;

fn main() {
    println!("Day result: {}", day::process(false));
    println!("Bonus result: {}", day::process(true));
}

struct Submarine {
    pub depth: i32,
    pub distance: i32,
    pub aim: i32,
}

impl Submarine {
    pub fn new() -> Submarine {
        Submarine {
            depth: 0,
            distance: 0,
            aim: 0,
        }
    }

    pub fn process(&mut self, cmd: Command) {
        match cmd {
            Command::Forward(val) => self.distance += val,
            Command::Up(val) => self.depth -= val,
            Command::Down(val) => self.depth += val,
            Command::Unknown => {}
        }
    }

    pub fn advanced_process(&mut self, cmd: Command) {
        match cmd {
            Command::Forward(val) => {
                self.distance += val;
                self.depth += val * self.aim;
            }
            Command::Up(val) => self.aim -= val,
            Command::Down(val) => self.aim += val,
            Command::Unknown => {}
        }
    }
}

enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
    Unknown,
}

impl Command {
    pub fn new(cmd_str: String) -> Command {
        let parts: Vec<&str> = cmd_str.split(' ').collect();
        let op = parts.first().unwrap_or(&"unknown");
        let val: i32 = parts.last().unwrap_or(&"0").parse().unwrap_or(0);

        match *op {
            "forward" => Command::Forward(val),
            "up" => Command::Up(val),
            "down" => Command::Down(val),
            _ => Command::Unknown,
        }
    }
}

mod day {
    use crate::{Command, Submarine};

    pub fn process(bonus: bool) -> i32 {
        let mut sub = Submarine::new();

        if let Ok(values) = super::tools::read_values::<String>("src/day2/values.txt") {
            for value in values {
                let cmd = Command::new(value);
                if bonus {
                    sub.advanced_process(cmd);
                } else {
                    sub.process(cmd);
                }
            }
        }

        sub.distance * sub.depth
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn day() {
        assert_eq!(super::day::process(false), 1936494);
    }

    #[test]
    fn bonus() {
        assert_eq!(super::day::process(true), 1997106066);
    }
}
