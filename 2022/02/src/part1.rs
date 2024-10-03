use crate::input::{Encrypted, Input, Shape};

#[derive(Debug, PartialEq, Eq)]
pub struct Output {
    score: u64,
}

impl Output {
    pub fn new(score: u64) -> Self {
        Self { score }
    }
}

impl core::fmt::Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.score)
    }
}

pub fn score_game(player: Shape, opponent: Shape) -> u64 {
    match (opponent, player) {
        (Shape::Rock, Shape::Rock) => 1 + 3,
        (Shape::Rock, Shape::Paper) => 2 + 6,
        (Shape::Rock, Shape::Scissors) => 3 + 0,
        (Shape::Paper, Shape::Rock) => 1 + 0,
        (Shape::Paper, Shape::Paper) => 2 + 3,
        (Shape::Paper, Shape::Scissors) => 3 + 6,
        (Shape::Scissors, Shape::Rock) => 1 + 6,
        (Shape::Scissors, Shape::Paper) => 2 + 0,
        (Shape::Scissors, Shape::Scissors) => 3 + 3,
    }
}

pub fn solve(input: &Input) -> Output {
    let mut score = 0;
    for &(opponent, encrypted) in input.guide.iter() {
        let player = match encrypted {
            Encrypted::X => Shape::Rock,
            Encrypted::Y => Shape::Paper,
            Encrypted::Z => Shape::Scissors,
        };
        score += score_game(player, opponent);
    }
    Output::new(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        struct TestCase {
            input: Input,
            expected: Output,
        }
        let test_cases = [
            TestCase {
                input: Input {
                    guide: vec![
                        (Shape::Rock, Encrypted::Y),
                        (Shape::Paper, Encrypted::X),
                        (Shape::Scissors, Encrypted::Z),
                    ],
                },
                expected: Output::new(15),
            },
            TestCase {
                input: Input {
                    guide: vec![
                        (Shape::Rock, Encrypted::X),
                        (Shape::Rock, Encrypted::Y),
                        (Shape::Rock, Encrypted::Z),
                        (Shape::Paper, Encrypted::X),
                        (Shape::Paper, Encrypted::Y),
                        (Shape::Paper, Encrypted::Z),
                        (Shape::Scissors, Encrypted::X),
                        (Shape::Scissors, Encrypted::Y),
                        (Shape::Scissors, Encrypted::Z),
                    ],
                },
                expected: Output::new(45),
            },
        ];
        for tc in test_cases {
            let result = solve(&tc.input);
            assert_eq!(result, tc.expected);
        }
    }
}
