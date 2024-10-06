#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Output {
    packages: Vec<char>,
}

impl Output {
    pub fn new(packages: Vec<char>) -> Self {
        Self { packages }
    }
}

impl core::fmt::Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.packages.iter().collect::<String>())
    }
}
