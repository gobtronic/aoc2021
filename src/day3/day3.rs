mod tools;

fn main() {
    let day = day::process();
    println!("Day result: {}", day);
}

mod day {
    pub fn process() -> u32 {
        if let Ok(values) = super::tools::read_values::<String>("src/day3/values.txt") {
            let int_values: Vec<i64> = values
                .iter()
                .flat_map(|value| i64::from_str_radix(value, 2))
                .collect();

            let mut gamma_bits: Vec<u8> = vec![];
            for i in 0..12 {
                let on_bits: &Vec<i64> = &int_values
                    .iter()
                    .map(|value| value >> i & 1)
                    .filter(|value| *value == 1)
                    .collect();

                if int_values.len() - on_bits.len() > on_bits.len() {
                    gamma_bits.push(0);
                } else {
                    gamma_bits.push(1);
                }
            }
            gamma_bits.reverse();

            let gamma_str: String = gamma_bits.iter().map(|value| value.to_string()).collect();
            let gamma_rate = u32::from_str_radix(&gamma_str, 2).unwrap_or(0);

            let epsilon_bits: Vec<u8> = gamma_bits
                .iter()
                .map(|b| {
                    if *b == 0 {
                        return 1;
                    }
                    0
                })
                .collect();
            let epsilon_str: String = epsilon_bits.iter().map(|value| value.to_string()).collect();
            let epsilon_rate = u32::from_str_radix(&epsilon_str, 2).unwrap_or(0);

            return gamma_rate * epsilon_rate;
        }

        0
    }
}
