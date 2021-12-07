mod tools;

fn main() {
    println!("Day result: {}", day::process(false));
    println!("Bonus result: {}", day::process(true));
}

pub struct BingoGrid {
    win_flag: bool,
    lines: Vec<Vec<(u32, bool)>>,
}

impl BingoGrid {
    pub fn new() -> BingoGrid {
        BingoGrid {
            win_flag: false,
            lines: vec![],
        }
    }

    pub fn insert_line(&mut self, line: String) {
        let line_numbers: Vec<u32> = line
            .split_ascii_whitespace()
            .map(|v| v.parse::<u32>().unwrap())
            .collect();
        let mut line = vec![];
        for number in line_numbers {
            line.push((number, false));
        }
        self.lines.push(line);
    }

    pub fn check_number(&mut self, number: u32) {
        for line in self.lines.iter_mut() {
            for (line_number, marked) in line.iter_mut() {
                if *line_number == number {
                    *marked = true;
                }
            }
        }
    }

    pub fn has_won(&mut self) -> bool {
        // Rows check
        for line in self.lines.iter() {
            if line.iter().map(|l| l.1).filter(|b| b == &true).count() == line.len() {
                self.win_flag = true;
                return true;
            }
        }

        // Columns check
        for (i, line) in self.lines.iter().enumerate() {
            let mut column: Vec<(u32, bool)> = vec![];
            for (j, _) in line.iter().enumerate() {
                column.push(self.lines[j][i]);
            }

            if column.iter().map(|l| l.1).filter(|b| b == &true).count() == column.len() {
                self.win_flag = true;
                return true;
            }
        }

        false
    }

    pub fn score(&self, last_marked: u32) -> u32 {
        let mut unmarked_sum: u32 = 0;
        for line in self.lines.iter() {
            unmarked_sum += line.iter().filter(|l| !l.1).map(|l| l.0).sum::<u32>();
        }

        unmarked_sum * last_marked
    }
}

impl Default for BingoGrid {
    fn default() -> Self {
        Self::new()
    }
}

mod day {
    use super::BingoGrid;

    pub fn process(last_wins: bool) -> u32 {
        if let Ok(values) = super::tools::read_values::<String>("src/day4/values.txt") {
            let mut numbers: Vec<u32> = vec![];
            let mut grids: Vec<BingoGrid> = vec![];

            for (i, line) in values.iter().enumerate() {
                match (i, line) {
                    (0, _) => numbers = line.split(',').flat_map(|v| v.parse::<u32>()).collect(),
                    (_, line) => {
                        if line.is_empty() {
                            grids.push(BingoGrid::default());
                        } else {
                            grids.last_mut().unwrap().insert_line(line.to_string());
                        }
                    }
                }
            }

            let mut score: u32 = 0;
            'nb_check: for number in numbers {
                for grid in grids.iter_mut().filter(|g| !g.win_flag) {
                    grid.check_number(number);
                    if grid.has_won() {
                        score = grid.score(number);
                        if !last_wins {
                            break 'nb_check;
                        }
                    }
                }
            }

            return score;
        }

        0
    }
}
