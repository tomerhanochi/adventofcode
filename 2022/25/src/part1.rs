use crate::{input::Input, output::Output, snafu::Snafu};

pub(crate) fn solve(input: &Input) -> Output {
    Output {
        total_fuel: Snafu(
            input
                .fuel_requirements
                .iter()
                .map(|Snafu(fuel_requirement)| fuel_requirement)
                .sum(),
        ),
    }
}
