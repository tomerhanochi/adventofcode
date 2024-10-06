use core::convert::Infallible;
use core::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Instruction {
    pub quantity: usize,
    pub source: usize,
    pub target: usize,
}

impl FromStr for Instruction {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(" ").collect::<Vec<_>>()[..] {
            ["move", quantity, "from", source, "to", target] => {
                Ok(Instruction {
                    quantity: quantity
                        .parse()
                        .expect("Quantity should be valid"),
                    source: source
                        .parse::<usize>()
                        .expect("Source should be valid.")
                        - 1,
                    target: target
                        .parse::<usize>()
                        .expect("Target should be valid.")
                        - 1,
                })
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Input {
    pub stacks: Vec<Vec<char>>,
    pub instructions: Vec<Instruction>,
}

impl FromStr for Input {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((stacks, instructions)) =
            s.trim_matches('\n').split_once("\n\n")
        else {
            unreachable!();
        };
        let stacks_count = stacks
            .lines()
            .last()
            .expect("Stacks should have at least 1 line")
            .split_whitespace()
            .last()
            .expect("Stacks should have at least 1 index")
            .parse()
            .expect("Stacks indecies should be numbers");
        let stacks = stacks.lines().rev().skip(1).fold(
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
        );

        let instructions = instructions
            .lines()
            .map(|instruction| {
                instruction.parse().expect("Instruction should be valid")
            })
            .collect();
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
                stacks: vec![
                    (vec!['Z', 'N']),
                    (vec!['M', 'C', 'D']),
                    (vec!['P']),
                ],
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
