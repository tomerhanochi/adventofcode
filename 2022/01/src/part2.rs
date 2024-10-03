use crate::{input::Input, part1::Output, queue::BoundedSortedQueue};

pub fn solve(input: &Input) -> Output {
    let calories_total_list = (*input.calories_nested_list)
        .iter()
        .map(|value| value.iter().sum::<u64>());
    let mut top = BoundedSortedQueue::<u64, 3>::default();
    for calories_total in calories_total_list {
        top.insert(calories_total);
    }
    Output::new(top.into_iter().sum())
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
            expected: Output::new(45000),
        }];
        for tc in test_cases {
            let result = solve(&tc.input);
            assert_eq!(result, tc.expected);
        }
    }
}
