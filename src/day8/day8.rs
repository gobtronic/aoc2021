mod tools;

fn main() {
    println!("Day result: {}", day::process());
}

mod day {
    pub fn process() -> u32 {
        if let Ok(values) = super::tools::read_values::<String>("src/day7/values.txt") {}

        0
    }
}
