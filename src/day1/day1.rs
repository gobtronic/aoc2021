
mod tools;

fn main() {
    if let Ok(values) = tools::read_values::<usize>("src/day1/values.txt") {
        let mut count = 0;
        let mut prev: Option<usize> = None;
        for value in values.iter() {
            match prev {
                Some(prev) => {
                    if value > &prev {
                        count += 1;
                    }
                },
                None => {}
            }

            prev = Some(*value);
        }

        println!("{}", count);
    }
}   