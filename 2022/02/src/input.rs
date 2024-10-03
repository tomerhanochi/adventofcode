use core::{panic, str::FromStr};
use std::convert::Infallible;

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

impl FromStr for Input {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut guide = Vec::new();
        for line in s.lines() {
            let Some((opponent, player)) = line.split_once(' ') else {
                panic!("Invalid line {:?}", line);
            };
            let opponent = match opponent {
                "A" => Shape::Rock,
                "B" => Shape::Paper,
                "C" => Shape::Scissors,
                _ => unreachable!(),
            };
            let player = match player {
                "X" => Encrypted::X,
                "Y" => Encrypted::Y,
                "Z" => Encrypted::Z,
                _ => unreachable!(),
            };
            guide.push((opponent, player));
        }
        Ok(Self { guide })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        struct TestCase<'a> {
            input: &'a str,
            expected: Result<Input, <Input as FromStr>::Err>,
        }
        let test_cases = [TestCase {
            input: "A Y\nB X\nC Z",
            expected: Ok(Input {
                guide: vec![
                    (Shape::Rock, Encrypted::Y),
                    (Shape::Paper, Encrypted::X),
                    (Shape::Scissors, Encrypted::Z),
                ],
            }),
        }];
        for tc in test_cases {
            let result = tc.input.parse::<Input>();
            assert_eq!(result, tc.expected);
        }
    }
}
