use crate::input::Input;
use crate::output::Output;

pub fn solve(input: &Input) -> Output {
    let mut stacks = input.stacks.clone();
    for instruction in input.instructions.iter() {
        let packages = {
            let source = &mut stacks[instruction.source];
            source.split_off(source.len() - instruction.quantity)
        };
        stacks[instruction.target].extend(packages);
    }
    Output::new(
        stacks
            .iter()
            .map(|stack| *stack.last().unwrap_or(&' '))
            .filter(|&package| package != ' ')
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use crate::input::Instruction;

    use super::*;

    #[test]
    fn test_solve() {
        struct TestCase {
            input: Input,
            expected: Output,
        }
        let test_cases = [TestCase {
            input: Input {
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
            },
            expected: Output::new(vec!['M', 'C', 'D']),
        }];
        for tc in test_cases {
            let result = solve(&tc.input);
            assert_eq!(result, tc.expected);
        }
    }
}
