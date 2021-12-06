
mod tools;

fn main() {
    println!("Day result: {}", day::process());
}

pub struct BingoGrid {
    numbers: [[u8; 8]; 8],
}

mod day {
    use super::BingoGrid;
    
    pub fn process() -> u32 {
        if let Ok(values) = super::tools::read_values::<String>("src/day4/values.txt") {
            let mut grids: Vec<BingoGrid> = vec![];
            let cur_grid: &BingoGrid;
            for (i, line) in values.iter().enumerate() {
                match (i, line) {
                    (0, _) => {

                    },
                    (_, "") => {

                    },
                    (_, line) => {

                    }
                    
                }
            }
        }
        0
    }
}