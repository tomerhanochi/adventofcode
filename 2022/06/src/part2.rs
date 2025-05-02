use crate::{input::Input, output::Output};

pub(crate) fn solve(input: &Input) -> Output {
    let length = 14;
    let marker_index = input
        .signal
        .windows(length)
        .enumerate()
        .find(|(_, window)| {
            for i in 0..(window.len() - 1) {
                for j in (i + 1)..window.len() {
                    if window[i] == window[j] {
                        return false;
                    }
                }
            }
            true
        })
        .map(|(index, _)| index)
        .unwrap();
    Output::new(marker_index + length)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        struct TestCase {
            input: Input,
            output: Output,
        }
        let test_cases = [
            TestCase {
                input: "mjqjpqmgbljsphdztnvjfqwrcgsmlb".parse().unwrap(),
                output: Output::new(19),
            },
            TestCase {
                input: "bvwbjplbgvbhsrlpgdmjqwftvncz".parse().unwrap(),
                output: Output::new(23),
            },
            TestCase {
                input: "nppdvjthqldpwncqszvftbrmjlhg".parse().unwrap(),
                output: Output::new(23),
            },
            TestCase {
                input: "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".parse().unwrap(),
                output: Output::new(29),
            },
            TestCase {
                input: "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".parse().unwrap(),
                output: Output::new(26),
            },
        ];
        for tc in test_cases {
            assert_eq!(solve(&tc.input), tc.output);
        }
    }
}
