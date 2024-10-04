use std::collections::HashSet;

use crate::{
    input::Input,
    part1::{get_priority, Output},
};

pub fn solve(input: &Input) -> Output {
    let mut priorities_sum = 0;
    for trio in input.rucksacks.chunks(3) {
        let Some(intersection) = trio
            .iter()
            .map(|(left, right)| {
                left.chars().chain(right.chars()).collect::<HashSet<_>>()
            })
            .reduce(|mut accumulator, rucksack| {
                accumulator.retain(|item| rucksack.contains(item));
                accumulator
            })
        else {
            unreachable!();
        };

        let Some(intersection) = intersection.iter().last() else {
            unreachable!();
        };

        priorities_sum += get_priority(intersection);
    }
    Output::new(priorities_sum)
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
        let test_cases = [TestCase {
            input: Input {
                rucksacks: vec![
                    ("vJrwpWtwJgWr".to_string(), "hcsFMMfFFhFp".to_string()),
                    (
                        "jqHRNqRjqzjGDLGL".to_string(),
                        "rsFMfFZSrLrFZsSL".to_string(),
                    ),
                    ("PmmdzqPrV".to_string(), "vPwwTWBwg".to_string()),
                    (
                        "wMqvLMZHhHMvwLH".to_string(),
                        "jbvcjnnSBnvTQFn".to_string(),
                    ),
                    ("ttgJtRGJ".to_string(), "QctTZtZT".to_string()),
                    ("CrZsJsPPZsGz".to_string(), "wwsLwLmpwMDw".to_string()),
                ],
            },
            expected: Output::new(70),
        }];
        for tc in test_cases {
            let result = solve(&tc.input);
            assert_eq!(result, tc.expected);
        }
    }
}
