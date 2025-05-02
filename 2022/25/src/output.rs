use std::fmt::Display;

use crate::snafu::Snafu;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Output {
    pub(crate) total_fuel: Snafu,
}

impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.total_fuel)
    }
}
