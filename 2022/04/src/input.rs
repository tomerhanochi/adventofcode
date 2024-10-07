use core::convert::Infallible;
use core::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct Range {
    start: u64,
    end: u64,
}

impl Range {
    #[cfg(test)]
    pub fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }

    pub fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    pub fn overlap(&self, other: &Self) -> bool {
        (self.start <= other.start && self.end >= other.start)
            || (self.start >= other.start && self.start <= other.end)
    }
}

impl FromStr for Range {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((start, end)) = s.split_once('-') else {
            unreachable!();
        };
        let Ok(start) = start.parse() else {
            unreachable!();
        };
        let Ok(end) = end.parse() else {
            unreachable!();
        };
        Ok(Self { start, end })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Input {
    pub section_assignments_pairs: Vec<(Range, Range)>,
}

impl FromStr for Input {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut section_assignments_pairs = Vec::new();
        for line in s.lines() {
            let Some((left, right)) = line.split_once(',') else {
                unreachable!();
            };
            let Ok(left) = left.parse() else {
                unreachable!();
            };
            let Ok(right) = right.parse() else {
                unreachable!();
            };
            section_assignments_pairs.push((left, right));
        }
        Ok(Self {
            section_assignments_pairs,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_contains() {
        struct TestCase {
            input: (Range, Range),
            expected: bool,
        }
        let test_cases = [
            TestCase {
                input: (Range::new(2, 4), Range::new(6, 8)),
                expected: false,
            },
            TestCase {
                input: (Range::new(2, 3), Range::new(4, 5)),
                expected: false,
            },
            TestCase {
                input: (Range::new(5, 7), Range::new(7, 9)),
                expected: false,
            },
            TestCase {
                input: (Range::new(6, 6), Range::new(4, 6)),
                expected: false,
            },
            TestCase {
                input: (Range::new(2, 8), Range::new(3, 7)),
                expected: true,
            },
            TestCase {
                input: (Range::new(4, 6), Range::new(6, 6)),
                expected: true,
            },
        ];
        for tc in test_cases {
            let result = tc.input.0.contains(&tc.input.1);
            assert_eq!(result, tc.expected);
        }
    }

    #[test]
    fn test_range_overlap() {
        struct TestCase {
            input: (Range, Range),
            expected: bool,
        }
        let test_cases = [
            TestCase {
                input: (Range::new(2, 4), Range::new(6, 8)),
                expected: false,
            },
            TestCase {
                input: (Range::new(2, 3), Range::new(4, 5)),
                expected: false,
            },
            TestCase {
                input: (Range::new(5, 7), Range::new(7, 9)),
                expected: true,
            },
            TestCase {
                input: (Range::new(2, 8), Range::new(3, 7)),
                expected: true,
            },
            TestCase {
                input: (Range::new(6, 6), Range::new(4, 6)),
                expected: true,
            },
            TestCase {
                input: (Range::new(2, 6), Range::new(4, 8)),
                expected: true,
            },
        ];
        for tc in test_cases {
            let result = tc.input.0.overlap(&tc.input.1);
            assert_eq!(result, tc.expected);
        }
    }

    #[test]
    fn test_range_from_str() {
        struct TestCase {
            input: &'static str,
            expected: Result<Range, <Range as FromStr>::Err>,
        }
        let test_cases = [
            TestCase {
                input: "2-4",
                expected: Ok(Range::new(2, 4)),
            },
            TestCase {
                input: "6-8",
                expected: Ok(Range::new(6, 8)),
            },
            TestCase {
                input: "6-6",
                expected: Ok(Range::new(6, 6)),
            },
        ];
        for tc in test_cases {
            let result = tc.input.parse();
            assert_eq!(result, tc.expected);
        }
    }

    #[test]
    fn test_input_from_str() {
        struct TestCase {
            input: &'static str,
            expected: Result<Input, <Input as FromStr>::Err>,
        }
        let test_cases = [TestCase {
            input: "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8",
            expected: Ok(Input {
                section_assignments_pairs: vec![
                    (Range::new(2, 4), Range::new(6, 8)),
                    (Range::new(2, 3), Range::new(4, 5)),
                    (Range::new(5, 7), Range::new(7, 9)),
                    (Range::new(2, 8), Range::new(3, 7)),
                    (Range::new(6, 6), Range::new(4, 6)),
                    (Range::new(2, 6), Range::new(4, 8)),
                ],
            }),
        }];
        for tc in test_cases {
            let result = tc.input.parse();
            assert_eq!(result, tc.expected);
        }
    }
}
