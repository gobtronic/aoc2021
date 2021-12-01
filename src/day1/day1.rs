mod tools;

fn main() {
    println!("Main result: {}", day::process());
    println!("Bonus result: {}", bonus::process());
}

mod day {
    pub fn process() -> i32 {
        if let Ok(values) = super::tools::read_values::<usize>("src/day1/values.txt") {
            let (_, count) = values.iter().fold((None, 0), |(last, count), val| {
                if let Some(last) = last {
                    if val > last {
                        return (Some(val), count + 1);
                    }
                }

                (Some(val), count)
            });

            return count;
        }

        0
    }
}

mod bonus {
    pub fn process() -> i32 {
        if let Ok(values) = super::tools::read_values::<usize>("src/day1/values.txt") {
            let (_, count) = values.windows(3).fold((None, 0), |(last, count), win| {
                let sum = win.iter().sum::<usize>();
                if let Some(last) = last {
                    if sum > last {
                        return (Some(sum), count + 1);
                    }
                }

                (Some(sum), count)
            });

            return count;
        }

        0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn day() {
        assert_eq!(super::day::process(), 1553);
    }

    #[test]
    fn bonus() {
        assert_eq!(super::bonus::process(), 1597);
    }
}
