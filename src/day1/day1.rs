
mod tools;

fn main() {
    if let Ok(values) = tools::read_values::<usize>("src/day1/values.txt") {
        println!("{:?}", values);
    }
}   