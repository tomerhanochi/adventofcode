use std::{
    convert::Infallible,
    ops::{Index, IndexMut},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Blizzard {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cell {
    Wall,
    Blizzards(Vec<Blizzard>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Valley {
    pub cells: Vec<Vec<Cell>>,
}

impl Index<(usize, usize)> for Valley {
    type Output = Cell;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.cells[index.1][index.0]
    }
}

impl IndexMut<(usize, usize)> for Valley {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.cells[index.1][index.0]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Input {
    pub valley: Valley,
}

impl FromStr for Input {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cells = Vec::new();
        for line in s.lines() {
            let mut temp = Vec::new();
            for char in line.chars() {
                temp.push(match char {
                    '#' => Cell::Wall,
                    '.' => Cell::Blizzards(vec![]),
                    '^' => Cell::Blizzards(vec![Blizzard::Up]),
                    '>' => Cell::Blizzards(vec![Blizzard::Right]),
                    'v' => Cell::Blizzards(vec![Blizzard::Down]),
                    '<' => Cell::Blizzards(vec![Blizzard::Left]),
                    _ => unreachable!(),
                });
            }
            cells.push(temp);
        }
        Ok(Self {
            valley: Valley { cells },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valley_from_str() {
        struct TestCase {
            input: &'static str,
            output: Input,
        }
        let test_cases = [TestCase {
            input: "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#",
            output: Input {
                valley: Valley {
                    cells: vec![
                        vec![
                            Cell::Wall,
                            Cell::Blizzards(vec![]),
                            Cell::Wall,
                            Cell::Wall,
                            Cell::Wall,
                            Cell::Wall,
                            Cell::Wall,
                            Cell::Wall,
                        ],
                        vec![
                            Cell::Wall,
                            Cell::Blizzards(vec![Blizzard::Right]),
                            Cell::Blizzards(vec![Blizzard::Right]),
                            Cell::Blizzards(vec![]),
                            Cell::Blizzards(vec![Blizzard::Left]),
                            Cell::Blizzards(vec![Blizzard::Up]),
                            Cell::Blizzards(vec![Blizzard::Left]),
                            Cell::Wall,
                        ],
                        vec![
                            Cell::Wall,
                            Cell::Blizzards(vec![]),
                            Cell::Blizzards(vec![Blizzard::Left]),
                            Cell::Blizzards(vec![]),
                            Cell::Blizzards(vec![]),
                            Cell::Blizzards(vec![Blizzard::Left]),
                            Cell::Blizzards(vec![Blizzard::Left]),
                            Cell::Wall,
                        ],
                        vec![
                            Cell::Wall,
                            Cell::Blizzards(vec![Blizzard::Right]),
                            Cell::Blizzards(vec![Blizzard::Down]),
                            Cell::Blizzards(vec![]),
                            Cell::Blizzards(vec![Blizzard::Right]),
                            Cell::Blizzards(vec![Blizzard::Left]),
                            Cell::Blizzards(vec![Blizzard::Right]),
                            Cell::Wall,
                        ],
                        vec![
                            Cell::Wall,
                            Cell::Blizzards(vec![Blizzard::Left]),
                            Cell::Blizzards(vec![Blizzard::Up]),
                            Cell::Blizzards(vec![Blizzard::Down]),
                            Cell::Blizzards(vec![Blizzard::Up]),
                            Cell::Blizzards(vec![Blizzard::Up]),
                            Cell::Blizzards(vec![Blizzard::Right]),
                            Cell::Wall,
                        ],
                        vec![
                            Cell::Wall,
                            Cell::Wall,
                            Cell::Wall,
                            Cell::Wall,
                            Cell::Wall,
                            Cell::Wall,
                            Cell::Blizzards(vec![]),
                            Cell::Wall,
                        ],
                    ],
                },
            },
        }];
        for tc in test_cases {
            assert_eq!(tc.input.parse::<Input>().unwrap(), tc.output);
        }
    }
}
