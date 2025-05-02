#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Output {
    pub(crate) marker_index: usize,
}

impl Output {
    pub(crate) fn new(marker_index: usize) -> Self {
        Self { marker_index }
    }
}

impl core::fmt::Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.marker_index)
    }
}
