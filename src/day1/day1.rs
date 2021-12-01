mod tools;

fn main() {
    println!("Main result: {}", day::process());
    println!("Bonus result: {}", bonus::process());
}

mod day {
    pub fn process() -> i32 {
        if let Ok(values) = super::tools::read_values::<usize>("src/day1/values.txt") {
            let mut count = 0;
            let mut prev: Option<usize> = None;
            for value in values.iter() {
                if let Some(prev) = prev {
                    if value > &prev {
                        count += 1;
                    }
                }

                prev = Some(*value);
            }

            return count;
        }

        0
    }
}

mod bonus {
    pub fn process() -> i32 {
        if let Ok(values) = super::tools::read_values::<usize>("src/day1/values.txt") {
            let mut count = 0;
            let mut prev_sum: Option<usize> = None;
            for (i, _) in values.iter().enumerate() {
                let i_sum = three_sum(i, &values);
                match i_sum {
                    Some(i_sum) => {
                        if let Some(prev_sum) = prev_sum {
                            if i_sum > prev_sum {
                                count += 1;
                            }
                        }

                        prev_sum = Some(i_sum);
                    }
                    None => break,
                }
            }

            return count;
        }

        0
    }

    fn three_sum(start: usize, values: &[usize]) -> Option<usize> {
        if start + 2 >= values.len() {
            return None;
        }

        Some(values[start] + values[start + 1_usize] + values[start + 2_usize])
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
