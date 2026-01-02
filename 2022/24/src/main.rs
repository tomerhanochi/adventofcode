mod input;
mod output;
mod part1;
mod part2;

use input::Input;

fn main() {
    let input = include_str!("input.txt")
        .parse::<Input>()
        .expect("Input should be valid");

    let output = part1::solve(&input);
    println!("Part 1: {}", output);

    // let output = part2::solve(&input);
    // println!("Part 2: {}", output);
}
