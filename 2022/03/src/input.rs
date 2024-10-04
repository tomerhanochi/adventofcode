use core::str::FromStr;
use std::convert::Infallible;

#[derive(Debug, PartialEq, Eq)]
pub struct Input {
    pub rucksacks: Vec<(String, String)>,
}

impl FromStr for Input {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rucksacks = Vec::new();
        for line in s.lines() {
            let (left, right) = line.split_at(line.len().div_euclid(2));
            rucksacks.push((left.to_string(), right.to_string()));
        }
        Ok(Input { rucksacks })
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
            input: "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw",
            expected: Ok(Input { rucksacks: vec![
                ("vJrwpWtwJgWr".to_string(), "hcsFMMfFFhFp".to_string()),
                ("jqHRNqRjqzjGDLGL".to_string(), "rsFMfFZSrLrFZsSL".to_string()),
                ("PmmdzqPrV".to_string(), "vPwwTWBwg".to_string()),
                ("wMqvLMZHhHMvwLH".to_string(), "jbvcjnnSBnvTQFn".to_string()),
                ("ttgJtRGJ".to_string(), "QctTZtZT".to_string()),
                ("CrZsJsPPZsGz".to_string(), "wwsLwLmpwMDw".to_string()),
                ]
            }),
        }];
        for tc in test_cases {
            let result = tc.input.parse::<Input>();
            assert_eq!(result, tc.expected);
        }
    }
}
