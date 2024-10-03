use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct Input {
    pub calories_nested_list: Vec<Vec<u64>>,
}

impl FromStr for Input {
    type Err = <u64 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut calories_nested_list = Vec::new();
        let mut calories_list = Vec::new();
        for line in s.lines() {
            if line.is_empty() {
                calories_nested_list.push(calories_list);
                calories_list = Vec::new();
            } else {
                let calories = line.parse::<u64>()?;
                calories_list.push(calories)
            }
        }
        if !calories_list.is_empty() {
            calories_nested_list.push(calories_list);
        }
        Ok(Input {
            calories_nested_list,
        })
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
            input: "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000",
            expected: Ok(Input {
                calories_nested_list: vec![
                    vec![1000, 2000, 3000],
                    vec![4000],
                    vec![5000, 6000],
                    vec![7000, 8000, 9000],
                    vec![10000],
                ],
            }),
        }];
        for tc in test_cases {
            let result = tc.input.parse::<Input>();
            assert_eq!(result, tc.expected);
        }
    }
}
