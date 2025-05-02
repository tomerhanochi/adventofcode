use std::{error::Error, fmt::Display, str::FromStr};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Snafu(pub(crate) i64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ParseSnafuError {
    InvalidCharacter,
}

impl Display for ParseSnafuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::InvalidCharacter => write!(
                f,
                "Encountered invalid character when parsing SNAFU number."
            ),
        }
    }
}

impl Error for ParseSnafuError {}

impl FromStr for Snafu {
    type Err = ParseSnafuError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut val = 0;
        let mut multiplier = 1;
        for char in s.chars().rev() {
            val += match char {
                '=' => -multiplier * 2,
                '-' => -multiplier,
                '0' => 0,
                '1' => multiplier,
                '2' => multiplier * 2,
                _ => return Err(Self::Err::InvalidCharacter),
            };
            multiplier *= 5;
        }
        Ok(Self(val))
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut power_of_5 = 1;
        let mut power_of_5_sum = power_of_5;
        let mut length = 1;
        while power_of_5 * 2 < self.0 {
            power_of_5 *= 5;
            power_of_5_sum += power_of_5;
            length += 1;
        }
        power_of_5_sum -= power_of_5;
        let mut accumulator = 0;

        write!(
            f,
            "{}",
            (0..length)
                .map(|_| {
                    let diff = self.0 - accumulator;
                    let digit = if diff <= -power_of_5 * 2 {
                        '='
                    } else if diff < -power_of_5 {
                        if -power_of_5 * 2 + power_of_5_sum * 2 >= diff {
                            '='
                        } else {
                            '-'
                        }
                    } else if diff < 0 {
                        if -power_of_5 + power_of_5_sum * 2 >= diff {
                            '-'
                        } else {
                            '0'
                        }
                    } else if diff < power_of_5 {
                        if power_of_5_sum * 2 >= diff {
                            '0'
                        } else {
                            '1'
                        }
                    } else if diff < power_of_5 * 2 {
                        if power_of_5 + power_of_5_sum * 2 >= diff {
                            '1'
                        } else {
                            '2'
                        }
                    } else {
                        '2'
                    };
                    accumulator += match digit {
                        '=' => -power_of_5 * 2,
                        '-' => -power_of_5,
                        '0' => 0,
                        '1' => power_of_5,
                        '2' => power_of_5 * 2,
                        _ => unreachable!(),
                    };
                    power_of_5 /= 5;
                    power_of_5_sum -= power_of_5;
                    digit
                })
                .skip_while(|digit| *digit == '0')
                .collect::<String>()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snafu_from_str() {
        struct TestCase {
            input: &'static str,
            output: Result<Snafu, ParseSnafuError>,
        }
        let test_cases = [
            TestCase {
                input: "1",
                output: Ok(Snafu(1)),
            },
            TestCase {
                input: "2",
                output: Ok(Snafu(2)),
            },
            TestCase {
                input: "1=",
                output: Ok(Snafu(3)),
            },
            TestCase {
                input: "1-",
                output: Ok(Snafu(4)),
            },
            TestCase {
                input: "10",
                output: Ok(Snafu(5)),
            },
            TestCase {
                input: "11",
                output: Ok(Snafu(6)),
            },
            TestCase {
                input: "12",
                output: Ok(Snafu(7)),
            },
            TestCase {
                input: "2=",
                output: Ok(Snafu(8)),
            },
            TestCase {
                input: "2-",
                output: Ok(Snafu(9)),
            },
            TestCase {
                input: "20",
                output: Ok(Snafu(10)),
            },
            TestCase {
                input: "1=0",
                output: Ok(Snafu(15)),
            },
            TestCase {
                input: "1-0",
                output: Ok(Snafu(20)),
            },
            TestCase {
                input: "1=11-2",
                output: Ok(Snafu(2022)),
            },
            TestCase {
                input: "1-0---0",
                output: Ok(Snafu(12345)),
            },
            TestCase {
                input: "1121-1110-1=0",
                output: Ok(Snafu(314159265)),
            },
            TestCase {
                input: "1=-0-2",
                output: Ok(Snafu(1747)),
            },
            TestCase {
                input: "12111",
                output: Ok(Snafu(906)),
            },
            TestCase {
                input: "2=0=",
                output: Ok(Snafu(198)),
            },
            TestCase {
                input: "21",
                output: Ok(Snafu(11)),
            },
            TestCase {
                input: "2=01",
                output: Ok(Snafu(201)),
            },
            TestCase {
                input: "111",
                output: Ok(Snafu(31)),
            },
            TestCase {
                input: "20012",
                output: Ok(Snafu(1257)),
            },
            TestCase {
                input: "112",
                output: Ok(Snafu(32)),
            },
            TestCase {
                input: "1=-1=",
                output: Ok(Snafu(353)),
            },
            TestCase {
                input: "1-12",
                output: Ok(Snafu(107)),
            },
            TestCase {
                input: "12",
                output: Ok(Snafu(7)),
            },
            TestCase {
                input: "1=",
                output: Ok(Snafu(3)),
            },
            TestCase {
                input: "122",
                output: Ok(Snafu(37)),
            },
        ];
        for tc in test_cases {
            assert_eq!(tc.input.parse(), tc.output);
        }
    }

    #[test]
    fn snafu_display() {
        struct TestCase {
            input: Snafu,
            output: String,
        }
        let test_cases = [
            TestCase {
                input: Snafu(1),
                output: "1".to_string(),
            },
            TestCase {
                input: Snafu(2),
                output: "2".to_string(),
            },
            TestCase {
                input: Snafu(3),
                output: "1=".to_string(),
            },
            TestCase {
                input: Snafu(4),
                output: "1-".to_string(),
            },
            TestCase {
                input: Snafu(5),
                output: "10".to_string(),
            },
            TestCase {
                input: Snafu(6),
                output: "11".to_string(),
            },
            TestCase {
                input: Snafu(7),
                output: "12".to_string(),
            },
            TestCase {
                input: Snafu(8),
                output: "2=".to_string(),
            },
            TestCase {
                input: Snafu(9),
                output: "2-".to_string(),
            },
            TestCase {
                input: Snafu(10),
                output: "20".to_string(),
            },
            TestCase {
                input: Snafu(15),
                output: "1=0".to_string(),
            },
            TestCase {
                input: Snafu(20),
                output: "1-0".to_string(),
            },
            TestCase {
                input: Snafu(2022),
                output: "1=11-2".to_string(),
            },
            TestCase {
                input: Snafu(12345),
                output: "1-0---0".to_string(),
            },
            TestCase {
                input: Snafu(314159265),
                output: "1121-1110-1=0".to_string(),
            },
            TestCase {
                input: Snafu(1747),
                output: "1=-0-2".to_string(),
            },
            TestCase {
                input: Snafu(906),
                output: "12111".to_string(),
            },
            TestCase {
                input: Snafu(198),
                output: "2=0=".to_string(),
            },
            TestCase {
                input: Snafu(11),
                output: "21".to_string(),
            },
            TestCase {
                input: Snafu(201),
                output: "2=01".to_string(),
            },
            TestCase {
                input: Snafu(31),
                output: "111".to_string(),
            },
            TestCase {
                input: Snafu(1257),
                output: "20012".to_string(),
            },
            TestCase {
                input: Snafu(32),
                output: "112".to_string(),
            },
            TestCase {
                input: Snafu(353),
                output: "1=-1=".to_string(),
            },
            TestCase {
                input: Snafu(107),
                output: "1-12".to_string(),
            },
            TestCase {
                input: Snafu(7),
                output: "12".to_string(),
            },
            TestCase {
                input: Snafu(3),
                output: "1=".to_string(),
            },
            TestCase {
                input: Snafu(37),
                output: "122".to_string(),
            },
        ];
        for tc in test_cases {
            assert_eq!(tc.input.to_string(), tc.output);
        }
    }
}
