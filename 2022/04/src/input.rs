use core::error::Error;
use core::fmt::Display;
use core::num::ParseIntError;
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseRangeError {
    InvalidFormat(String),
    ParseStart(ParseIntError),
    ParseEnd(ParseIntError),
}

impl Display for ParseRangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::InvalidFormat(ref line) => {
                write!(f, "invalid format for range: {}", line)
            }
            Self::ParseStart(_) => write!(f, "failed to parse start of range"),
            Self::ParseEnd(_) => write!(f, "failed to parse end of range"),
        }
    }
}

impl Error for ParseRangeError {
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            Self::InvalidFormat(_) => None,
            Self::ParseStart(ref err) => Some(err),
            Self::ParseEnd(ref err) => Some(err),
        }
    }
}

impl FromStr for Range {
    type Err = ParseRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((start, end)) = s.split_once('-') else {
            return Err(ParseRangeError::InvalidFormat(s.to_string()));
        };
        let start = start.parse().map_err(ParseRangeError::ParseStart)?;
        let end = end.parse().map_err(ParseRangeError::ParseEnd)?;
        Ok(Self { start, end })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Input {
    pub section_assignments_pairs: Vec<(Range, Range)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseInputError {
    InvalidFormat(String),
    ParseRange(ParseRangeError),
}

impl Display for ParseInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::InvalidFormat(ref line) => {
                write!(f, "invalid format for input line: {}", line)
            }
            Self::ParseRange(_) => {
                write!(f, "failed to parse range")
            }
        }
    }
}

impl Error for ParseInputError {
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            Self::InvalidFormat(_) => None,
            Self::ParseRange(ref err) => Some(err),
        }
    }
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut section_assignments_pairs = Vec::new();
        for line in s.lines() {
            let Some((left, right)) = line.split_once(',') else {
                return Err(ParseInputError::InvalidFormat(line.to_string()));
            };
            let left = left.parse().map_err(ParseInputError::ParseRange)?;
            let right = right.parse().map_err(ParseInputError::ParseRange)?;
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
