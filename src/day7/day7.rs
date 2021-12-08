mod tools;

fn main() {
    println!("Day result: {}", day::process());
    println!("Bonus result: {}", bonus::process());
}

mod day {
    pub fn process() -> u32 {
        if let Ok(values) = super::tools::read_values::<String>("src/day7/values.txt") {
            let mut values: Vec<u32> = values
                .first()
                .unwrap()
                .split(',')
                .flat_map(|f| f.parse())
                .collect();
            values.sort_unstable();

            let median: u32;
            if values.len() % 2 == 0 {
                let lower_i = (values.len() as f32 / 2_f32).floor() - 1_f32;
                let first = values[lower_i as usize];
                let second = values[lower_i as usize + 1];
                median = (first + second) / 2;
            } else {
                let i = (values.len() as f32 / 2_f32).ceil();
                median = values[i as usize];
            }

            return values.iter().fold(0_u32, |count, f| {
                count + (*f as i64 - median as i64).abs() as u32
            });
        }

        0
    }
}

mod bonus {
    pub fn process() -> u32 {
        if let Ok(values) = super::tools::read_values::<String>("src/day7/values.txt") {
            let values: Vec<u32> = values
                .first()
                .unwrap()
                .split(',')
                .flat_map(|f| f.parse())
                .collect();
            let medium: u32 = values.iter().sum::<u32>() / values.len() as u32;

            return values.iter().fold(0_u32, |count, f| {
                let mut fuel_sum = 0;
                let dist = (*f as i64 - medium as i64).abs() as u32;
                for i in 1..=dist {
                    fuel_sum += i;
                }

                count + fuel_sum
            });
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use crate::{bonus, day};

    #[test]
    fn day_mock() {
        let values: Vec<u32> = vec![0, 1, 1, 2, 2, 2, 4, 7, 14, 16];
        let median: u32;
        if values.len() % 2 == 0 {
            let lower_i = (values.len() as f32 / 2_f32).floor() - 1_f32;
            let first = values[lower_i as usize];
            let second = values[lower_i as usize + 1];
            median = (first + second) / 2;
        } else {
            let i = (values.len() as f32 / 2_f32).ceil();
            median = values[i as usize];
        }

        assert_eq!(
            values.iter().fold(0_u32, |count, f| count
                + (*f as i64 - median as i64).abs() as u32),
            37
        );
    }

    #[test]
    fn day() {
        assert_eq!(day::process(), 331067);
    }

    #[test]
    fn bonus() {
        assert_eq!(bonus::process(), 92881128);
    }
}
