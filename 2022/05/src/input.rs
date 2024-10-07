use core::str::FromStr;
use std::error::Error;
use std::fmt::Display;
use std::num::ParseIntError;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Stacks(Vec<Vec<char>>);

#[cfg(test)]
impl Stacks {
    pub fn new(stacks: Vec<Vec<char>>) -> Self {
        Self(stacks)
    }
}

impl Deref for Stacks {
    type Target = Vec<Vec<char>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Stacks {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseStacksError {
    Empty,
    MissingIndex,
    ParseIndex(ParseIntError),
}

impl Display for ParseStacksError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Empty => write!(f, "cannot parse stacks from empty string"),
            Self::MissingIndex => {
                write!(f, "cannot parse stacks with missing index")
            }
            Self::ParseIndex(_) => write!(f, "failed to parse index"),
        }
    }
}

impl Error for ParseStacksError {
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            Self::Empty => None,
            Self::MissingIndex => None,
            Self::ParseIndex(ref err) => Some(err),
        }
    }
}

impl FromStr for Stacks {
    type Err = ParseStacksError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stacks_count = s
            .lines()
            .last()
            .ok_or(ParseStacksError::Empty)?
            .split_whitespace()
            .last()
            .ok_or(ParseStacksError::MissingIndex)?
            .parse()
            .map_err(ParseStacksError::ParseIndex)?;
        Ok(Self(s.lines().rev().skip(1).fold(
            (0..stacks_count).map(|_| Vec::new()).collect::<Vec<_>>(),
            |mut acc, line| {
                for (stack_idx, package) in line
                    .trim_end()
                    .chars()
                    .skip("[".len())
                    .step_by("] [X".len())
                    .enumerate()
                    .filter(|(_, package)| *package != ' ')
                    .collect::<Vec<_>>()
                {
                    acc[stack_idx].push(package);
                }
                acc
            },
        )))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Instruction {
    pub quantity: usize,
    pub source: usize,
    pub target: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseInstructionError {
    InvalidFormat(String),
    ParseQuantity(ParseIntError),
    ParseSource(ParseIntError),
    ParseTarget(ParseIntError),
}

impl Display for ParseInstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::InvalidFormat(ref line) => {
                write!(f, "invalid format for instruction: {}", line)
            }
            Self::ParseQuantity(_) => write!(f, "failed to parse quantity"),
            Self::ParseSource(_) => write!(f, "failed to parse source"),
            Self::ParseTarget(_) => write!(f, "failed to parse target"),
        }
    }
}

impl Error for ParseInstructionError {
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            Self::InvalidFormat(_) => None,
            Self::ParseQuantity(ref err) => Some(err),
            Self::ParseSource(ref err) => Some(err),
            Self::ParseTarget(ref err) => Some(err),
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(" ").collect::<Vec<_>>()[..] {
            ["move", quantity, "from", source, "to", target] => {
                Ok(Instruction {
                    quantity: quantity
                        .parse()
                        .map_err(ParseInstructionError::ParseQuantity)?,
                    source: source
                        .parse::<usize>()
                        .map_err(ParseInstructionError::ParseSource)?
                        - 1,
                    target: target
                        .parse::<usize>()
                        .map_err(ParseInstructionError::ParseTarget)?
                        - 1,
                })
            }
            _ => Err(ParseInstructionError::InvalidFormat(s.to_string())),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Input {
    pub stacks: Stacks,
    pub instructions: Vec<Instruction>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseInputError {
    InvalidFormat,
    ParseStacks(ParseStacksError),
    ParseInstructions(ParseInstructionError),
}

impl Display for ParseInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::InvalidFormat => {
                write!(f, "cannot parse input with invalid format")
            }
            Self::ParseStacks(_) => write!(f, "cannot parse stacks"),
            Self::ParseInstructions(_) => {
                write!(f, "cannot parse instructions")
            }
        }
    }
}

impl Error for ParseInputError {
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            Self::InvalidFormat => None,
            Self::ParseStacks(ref err) => Some(err),
            Self::ParseInstructions(ref err) => Some(err),
        }
    }
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (stacks, instructions) = s
            .trim_matches('\n')
            .split_once("\n\n")
            .ok_or(ParseInputError::InvalidFormat)?;

        let stacks = stacks
            .parse::<Stacks>()
            .map_err(ParseInputError::ParseStacks)?;

        let instructions = instructions
            .lines()
            .map(|instruction| {
                instruction
                    .parse()
                    .map_err(ParseInputError::ParseInstructions)
            })
            .collect::<Result<_, _>>()?;
        Ok(Input {
            stacks,
            instructions,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_from_str() {
        struct TestCase {
            input: &'static str,
            expected: Result<Instruction, <Instruction as FromStr>::Err>,
        }
        let test_cases = [
            TestCase {
                input: "move 1 from 2 to 1",
                expected: Ok(Instruction {
                    quantity: 1,
                    source: 1,
                    target: 0,
                }),
            },
            TestCase {
                input: "move 3 from 1 to 3",
                expected: Ok(Instruction {
                    quantity: 3,
                    source: 0,
                    target: 2,
                }),
            },
            TestCase {
                input: "move 2 from 2 to 1",
                expected: Ok(Instruction {
                    quantity: 2,
                    source: 1,
                    target: 0,
                }),
            },
            TestCase {
                input: "move 1 from 1 to 2",
                expected: Ok(Instruction {
                    quantity: 1,
                    source: 0,
                    target: 1,
                }),
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
            input: "
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
            expected: Ok(Input {
                stacks: Stacks::new(vec![
                    (vec!['Z', 'N']),
                    (vec!['M', 'C', 'D']),
                    (vec!['P']),
                ]),
                instructions: vec![
                    Instruction {
                        quantity: 1,
                        source: 1,
                        target: 0,
                    },
                    Instruction {
                        quantity: 3,
                        source: 0,
                        target: 2,
                    },
                    Instruction {
                        quantity: 2,
                        source: 1,
                        target: 0,
                    },
                    Instruction {
                        quantity: 1,
                        source: 0,
                        target: 1,
                    },
                ],
            }),
        }];
        for tc in test_cases {
            let result = tc.input.parse();
            assert_eq!(result, tc.expected);
        }
    }
}
