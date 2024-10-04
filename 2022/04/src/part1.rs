use crate::input::Input;

#[derive(Debug, PartialEq, Eq)]
pub struct Output {
    num_of_contains: u64,
}

impl Output {
    pub fn new(num_of_contains: u64) -> Self {
        Self { num_of_contains }
    }
}

impl core::fmt::Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.num_of_contains)
    }
}

pub fn solve(input: &Input) -> Output {
    Output::new(
        input
            .section_assignments_pairs
            .iter()
            .map(|(left, right)| left.contains(right) || right.contains(left))
            .filter(|&predicate| predicate)
            .count() as u64,
    )
}

#[cfg(test)]
mod tests {
    use crate::input::Range;

    use super::*;

    #[test]
    fn test_solve() {
        struct TestCase {
            input: Input,
            expected: Output,
        }
        let test_cases = [TestCase {
            input: Input {
                section_assignments_pairs: vec![
                    (Range::new(2, 4), Range::new(6, 8)),
                    (Range::new(2, 3), Range::new(4, 5)),
                    (Range::new(5, 7), Range::new(7, 9)),
                    (Range::new(2, 8), Range::new(3, 7)),
                    (Range::new(6, 6), Range::new(4, 6)),
                    (Range::new(2, 6), Range::new(4, 8)),
                ],
            },
            expected: Output::new(2),
        }];
        for tc in test_cases {
            let result = solve(&tc.input);
            assert_eq!(result, tc.expected);
        }
    }
}
