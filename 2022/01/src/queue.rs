#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BoundedSortedQueue<T, const N: usize>([T; N]);

impl<T: Ord, const N: usize> From<[T; N]> for BoundedSortedQueue<T, N> {
    fn from(mut value: [T; N]) -> Self {
        value.sort();
        Self(value)
    }
}

impl<T: Default + Ord, const N: usize> Default for BoundedSortedQueue<T, N> {
    fn default() -> Self {
        Self(core::array::from_fn(|_| T::default()))
    }
}

impl<T, const N: usize> IntoIterator for BoundedSortedQueue<T, N> {
    type Item = T;
    type IntoIter = core::array::IntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T: Ord, const N: usize> BoundedSortedQueue<T, N> {
    pub fn insert(&mut self, item: T) {
        for i in 0..N {
            match item.cmp(&self.0[i]) {
                core::cmp::Ordering::Less | core::cmp::Ordering::Equal => {
                    if i == 0 {
                        return;
                    }
                    for j in 1..i {
                        self.0.swap(j, j - 1);
                    }
                    self.0[i - 1] = item;
                }
                core::cmp::Ordering::Greater => {
                    if i != N - 1 {
                        continue;
                    }
                    for j in 1..=i {
                        self.0.swap(j, j - 1);
                    }
                    self.0[i] = item;
                }
            }
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue_insert() {
        type Inner = u8;
        const QUEUE_SIZE: usize = 5;
        struct TestCase {
            input: [Inner; 10],
            expected: BoundedSortedQueue<Inner, QUEUE_SIZE>,
        }
        let test_cases = [
            TestCase {
                input: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
                expected: [6, 7, 8, 9, 10].into(),
            },
            TestCase {
                input: [99, 1, 98, 2, 97, 3, 96, 4, 95, 5],
                expected: [95, 96, 97, 98, 99].into(),
            },
            TestCase {
                input: [1, 99, 2, 99, 3, 99, 4, 99, 5, 99],
                expected: [99, 99, 99, 99, 99].into(),
            },
        ];
        for tc in test_cases {
            let mut result = BoundedSortedQueue::<Inner, QUEUE_SIZE>::default();
            for item in tc.input {
                result.insert(item);
            }
            assert_eq!(result, tc.expected);
        }
    }
}
