mod tools;

fn main() {
    println!("Day result: {}", day::process());
}

pub struct Lanternfish {
    pub timer: u8,
    pub children: Vec<Lanternfish>,
}

impl Default for Lanternfish {
    fn default() -> Self {
        Lanternfish {
            timer: 8,
            children: vec![],
        }
    }
}

impl Lanternfish {
    pub fn another_day(&mut self) {
        self.children.iter_mut().for_each(|f| f.another_day());

        if self.timer == 0 {
            self.timer = 6;
            self.children.push(Lanternfish::default());
        } else {
            self.timer -= 1
        }
    }

    pub fn nb_descendants(&self) -> usize {
        self.children
            .iter()
            .fold(self.children.len(), |last, f| last + f.nb_descendants())
    }
}

pub struct ParseError;

impl From<&str> for Lanternfish {
    fn from(s: &str) -> Self {
        let timer: u8 = s.parse::<u8>().unwrap();
        Lanternfish {
            timer,
            children: vec![],
        }
    }
}

mod day {
    use crate::Lanternfish;

    pub fn process() -> usize {
        if let Ok(values) = super::tools::read_values::<String>("src/day6/values.txt") {
            let mut values: Vec<Lanternfish> = values
                .first()
                .unwrap()
                .split(',')
                .flat_map(|f| f.try_into())
                .collect();
            for _ in 0..80 {
                for fish in values.iter_mut() {
                    fish.another_day();
                }
            }

            return values.len() + values.iter().fold(0, |last, f| last + f.nb_descendants());
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use crate::Lanternfish;

    #[test]
    fn mathematics_how_do_they_work() {
        let mut values: Vec<Lanternfish> = vec![Lanternfish {
            timer: 3,
            children: vec![],
        }];

        let total = (((80 - 8) / 6) as f32).floor().powf(2_f32);
        assert_eq!(total, 1154_f32);
    }

    #[test]
    fn day() {
        let mut values: Vec<Lanternfish> = vec![
            Lanternfish {
                timer: 3,
                children: vec![],
            },
            Lanternfish {
                timer: 4,
                children: vec![],
            },
            Lanternfish {
                timer: 3,
                children: vec![],
            },
            Lanternfish {
                timer: 1,
                children: vec![],
            },
            Lanternfish {
                timer: 2,
                children: vec![],
            },
        ];

        for _ in 0..80 {
            for fish in values.iter_mut() {
                fish.another_day();
            }
        }

        assert_eq!(
            values.len() + values.iter().fold(0, |last, f| last + f.nb_descendants()),
            5934
        );
    }

    #[test]
    fn bonus() {
        let mut values: Vec<Lanternfish> = vec![
            Lanternfish {
                timer: 3,
                children: vec![],
            },
            Lanternfish {
                timer: 4,
                children: vec![],
            },
            Lanternfish {
                timer: 3,
                children: vec![],
            },
            Lanternfish {
                timer: 1,
                children: vec![],
            },
            Lanternfish {
                timer: 2,
                children: vec![],
            },
        ];

        for _ in 0..256 {
            for fish in values.iter_mut() {
                fish.another_day();
            }
        }

        assert_eq!(
            values.len() + values.iter().fold(0, |last, f| last + f.nb_descendants()),
            26984457539
        );
    }
}
