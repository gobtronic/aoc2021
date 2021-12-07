mod tools;

fn main() {
    println!("Day result: {}", day::process());
    println!("Bonus result: {}", bonus::process());
}

mod day {
    pub fn process() -> u32 {
        if let Ok(values) = super::tools::read_values::<String>("src/day3/values.txt") {
            let int_values: Vec<u64> = values
                .iter()
                .flat_map(|value| u64::from_str_radix(value, 2))
                .collect();

            let mut gamma_bits: Vec<u8> = vec![];
            for i in 0..12 {
                // Get all bits that are on in the column i
                let on_bits: &Vec<u64> = &int_values
                    .iter()
                    .map(|value| value >> i & 1)
                    .filter(|value| *value == 1)
                    .collect();

                // Test if there is more off bits than on bits
                if int_values.len() - on_bits.len() > on_bits.len() {
                    gamma_bits.push(0);
                } else {
                    gamma_bits.push(1);
                }
            }
            gamma_bits.reverse();

            let gamma_str: String = gamma_bits.iter().map(|value| value.to_string()).collect();
            let gamma_rate = u32::from_str_radix(&gamma_str, 2).unwrap_or(0);

            // Toggle gamma_bits to build epsilon_bits
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

mod bonus {
    pub fn process() -> u64 {
        if let Ok(values) = super::tools::read_values::<String>("src/day3/values.txt") {
            let int_values: Vec<u64> = values
                .iter()
                .flat_map(|value| u64::from_str_radix(value, 2))
                .collect();

            let oxygen_values = recursive_common_bit(0, int_values.clone(), false);
            let oxygen = oxygen_values.first().unwrap_or(&0);
            let co2_values = recursive_common_bit(0, int_values, true);
            let co2 = co2_values.first().unwrap_or(&0);

            return oxygen * co2;
        }

        0
    }

    fn recursive_common_bit(pos: i32, values: Vec<u64>, inverted: bool) -> Vec<u64> {
        let on_bits: Vec<u64> = values
            .iter()
            // Some examples for pos == 0:
            // Masks 1110_0110_0000 to 1000_0000_0000
            // Or 0101_0000_0000 to 0000_0000_0000
            .map(|value| value & (1 << 11 - pos))
            // For pos == 0:
            // Filters all values that are == 2^11 (others are == 0)
            .filter(|value| *value == 2_u64.pow(11 - pos as u32))
            // We have the number of on bits (== 1)
            .collect();

        // What dominant bit are we looking for
        let off_b_count = values.len() - on_bits.len();
        let dominant_b: u8;
        if inverted {
            if on_bits.len() < off_b_count {
                dominant_b = 1;
            } else {
                dominant_b = 0;
            }
        } else {
            if on_bits.len() >= off_b_count {
                dominant_b = 1;
            } else {
                dominant_b = 0;
            }
        }

        // Same operation as before, we mask values and get all values that have the dominant bat required at `pos`
        let filtered_values: Vec<u64> = values
            .into_iter()
            .filter(|value| {
                let masked_value = value & (1 << 11 - pos);
                if dominant_b == 0 {
                    masked_value == 0
                } else {
                    masked_value == 2_u64.pow(11 - pos as u32)
                }
            })
            .collect();

        // Recursive until last pos (== 11)
        if pos != 11 && filtered_values.len() > 1 {
            return recursive_common_bit(pos + 1, filtered_values, inverted);
        }

        filtered_values
    }
}
