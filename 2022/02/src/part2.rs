use crate::{
    input::{Encrypted, Input, Shape},
    part1::{score_game, Output},
};

pub fn solve(input: &Input) -> Output {
    let mut score = 0;
    for &(opponent, encrypted) in input.guide.iter() {
        let player = match encrypted {
            Encrypted::X => match opponent {
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper,
            },
            Encrypted::Y => opponent,
            Encrypted::Z => match opponent {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock,
            },
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
                expected: Output::new(12),
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
