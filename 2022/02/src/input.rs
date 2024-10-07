use core::{error::Error, fmt::Display, str::FromStr};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Encrypted {
    X,
    Y,
    Z,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Input {
    pub guide: Vec<(Shape, Encrypted)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseInputError {
    MatchFormat { line: String },
    Opponent { opponent: String },
    Encrypted { encrypted: String },
}

impl Display for ParseInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::MatchFormat { ref line } => {
                write!(f, "match isn't in the correct format: {}", line)
            }
            Self::Opponent { ref opponent } => {
                write!(f, "opponent isn't one of A, B or C: {}", opponent)
            }
            Self::Encrypted { ref encrypted } => {
                write!(f, "encrypted isn't one of X, Y or Z: {}", encrypted)
            }
        }
    }
}

impl Error for ParseInputError {}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut guide = Vec::new();
        for line in s.lines() {
            let Some((opponent, encrypted)) = line.split_once(' ') else {
                return Err(ParseInputError::MatchFormat {
                    line: line.to_string(),
                });
            };
            let opponent = match opponent {
                "A" => Shape::Rock,
                "B" => Shape::Paper,
                "C" => Shape::Scissors,
                _ => {
                    return Err(ParseInputError::Opponent {
                        opponent: opponent.to_string(),
                    })
                }
            };
            let encrypted = match encrypted {
                "X" => Encrypted::X,
                "Y" => Encrypted::Y,
                "Z" => Encrypted::Z,
                _ => {
                    return Err(ParseInputError::Encrypted {
                        encrypted: encrypted.to_string(),
                    })
                }
            };
            guide.push((opponent, encrypted));
        }
        Ok(Self { guide })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        struct TestCase {
            input: &'static str,
            expected: Result<Input, <Input as FromStr>::Err>,
        }
        let test_cases = [
            TestCase {
                input: "A Y\nB X\nC Z",
                expected: Ok(Input {
                    guide: vec![
                        (Shape::Rock, Encrypted::Y),
                        (Shape::Paper, Encrypted::X),
                        (Shape::Scissors, Encrypted::Z),
                    ],
                }),
            },
            TestCase {
                input: "A Y\nxxx\nC Z",
                expected: Err(ParseInputError::MatchFormat {
                    line: "xxx".to_string(),
                }),
            },
            TestCase {
                input: "A Y\nD Z\nC Z",
                expected: Err(ParseInputError::Opponent {
                    opponent: "D".to_string(),
                }),
            },
            TestCase {
                input: "A Y\nB T\nC Z",
                expected: Err(ParseInputError::Encrypted {
                    encrypted: "T".to_string(),
                }),
            },
        ];
        for tc in test_cases {
            let result = tc.input.parse::<Input>();
            assert_eq!(result, tc.expected);
        }
    }
}
