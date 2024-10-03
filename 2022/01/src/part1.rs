use crate::input::Input;

#[derive(Debug, Eq, PartialEq)]
pub struct Output {
    max_calories: u64,
}

impl Output {
    pub fn new(max_calories: u64) -> Self {
        Self { max_calories }
    }
}

impl core::fmt::Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.max_calories)
    }
}

pub fn solve(input: &Input) -> Output {
    Output::new(
        (*input.calories_nested_list)
            .iter()
            .map(|value| value.iter().sum::<u64>())
            .max()
            .unwrap_or(0),
    )
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
                calories_nested_list: vec![
                    vec![1000, 2000, 3000],
                    vec![4000],
                    vec![5000, 6000],
                    vec![7000, 8000, 9000],
                    vec![10000],
                ],
            },
            expected: Output::new(24000),
        }];
        for tc in test_cases {
            let result = solve(&tc.input);
            assert_eq!(result, tc.expected);
        }
    }
}
