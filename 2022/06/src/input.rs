use std::{convert::Infallible, str::FromStr};

#[derive(Debug)]
pub(crate) struct Input {
    pub(crate) signal: Vec<char>,
}

impl FromStr for Input {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            signal: s.chars().collect(),
        })
    }
}
