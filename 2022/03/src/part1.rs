use std::collections::HashSet;

use crate::input::Input;

#[derive(Debug, PartialEq, Eq)]
pub struct Output {
    priorities_sum: u64,
}

impl Output {
    pub fn new(priorities_sum: u64) -> Self {
        Self { priorities_sum }
    }
}

impl core::fmt::Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.priorities_sum)
    }
}

pub fn get_priority(item: &char) -> u64 {
    // Priority is mapped:
    // a-z -> 1-26
    // A-Z -> 27-52
    if item.is_ascii_lowercase() {
        // a-z is 61-7a (base 16) in UTF-8
        // To get 1-26 (base 10): Take codepoint as uint, and subtract 60 (base 16)
        *item as u64 - 6 * 16
    } else if item.is_ascii_uppercase() {
        // A-Z is 41-5a (base 16) in UTF-8
        // To get 27-52 (base 10): Take codepoint as uint, subtract 40 (base 16) and add 26 (base 10)
        *item as u64 - 4 * 16 + 26
    } else {
        0
    }
}

pub fn solve(input: &Input) -> Output {
    let mut priorities_sum = 0;

    for (left, right) in input.rucksacks.iter() {
        let left = left.chars().collect::<HashSet<_>>();
        let right = right.chars().collect::<HashSet<_>>();
        let Some(item) = left.intersection(&right).last() else {
            unreachable!();
        };
        priorities_sum += get_priority(item);
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
            expected: Output::new(157),
        }];
        for tc in test_cases {
            let result = solve(&tc.input);
            assert_eq!(result, tc.expected);
        }
    }
}
