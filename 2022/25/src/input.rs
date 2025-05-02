use std::{error::Error, fmt::Display, str::FromStr};

use crate::snafu::{ParseSnafuError, Snafu};

pub(crate) struct Input {
    pub(crate) fuel_requirements: Vec<Snafu>,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum ParseInputError {
    ParseSnafuError(ParseSnafuError),
}

impl Display for ParseInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseSnafuError(err) => write!(f, "{}", err),
        }
    }
}

impl Error for ParseInputError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ParseSnafuError(err) => Some(err),
        }
    }
}

impl From<ParseSnafuError> for ParseInputError {
    fn from(value: ParseSnafuError) -> Self {
        Self::ParseSnafuError(value)
    }
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            fuel_requirements: s
                .lines()
                .map(|line| line.parse())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}
