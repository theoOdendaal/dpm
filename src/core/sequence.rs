//! Sequence creation.

//  --- Structs

#[derive(Debug, Default)]
pub struct Sequence<A> {
    elements: Vec<A>,
}

//  --- Trait implementations: Blanket

impl<A> Sequence<A> {
    pub fn from_step<B>(lower: A, upper: A, step: B) -> Self
    where
        A: Copy
            + std::cmp::PartialOrd
            + std::ops::Sub<B, Output = A>
            + std::ops::Add<B, Output = A>,
        B: Copy,
    {
        let min_value = if lower < upper { lower } else { upper };
        let mut max_value = if upper > lower { upper } else { lower };

        let mut sequence: Vec<A> = Vec::new();

        while max_value > min_value {
            sequence.push(max_value);

            let last_value = min_value + step;
            max_value = if max_value > last_value {
                max_value
            } else {
                last_value
            };
            max_value = max_value - step
        }

        sequence.push(min_value);
        sequence.reverse();
        sequence.shrink_to_fit();
        Self { elements: sequence }
    }
}

//  --- Trait implementations: Standard library

impl<A> From<Sequence<A>> for Vec<A>
where
    A: Copy,
{
    fn from(value: Sequence<A>) -> Self {
        value.elements.to_vec()
    }
}

impl<A> From<Vec<A>> for Sequence<A> {
    fn from(value: Vec<A>) -> Self {
        Self { elements: value }
    }
}

impl<A> IntoIterator for Sequence<A> {
    type Item = A;
    type IntoIter = std::vec::IntoIter<A>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

impl<A> std::ops::Index<usize> for Sequence<A> {
    type Output = A;

    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}

//  --- Unit tests

#[cfg(test)]
mod test_frequency {

    use super::*;

    #[test]
    fn test_i32_sequence() {
        let lower = 1;
        let upper = 11;
        let step = 3;

        let sequence: Vec<i32> = Sequence::from_step(upper, lower, step).into();
        let expected = vec![1, 2, 5, 8, 11];
        assert_eq!(sequence, expected)
    }

    #[test]
    fn test_u32_sequence() {
        let lower: u32 = 1;
        let upper: u32 = 11;
        let step: u32 = 3;

        let sequence: Vec<u32> = Sequence::from_step(upper, lower, step).into();
        let expected = vec![1, 2, 5, 8, 11];
        assert_eq!(sequence, expected)
    }

    #[test]
    fn test_f64_sequence() {
        let lower = 1.0;
        let upper = 11.0;
        let step = 3.0;

        let sequence: Vec<f64> = Sequence::from_step(upper, lower, step).into();
        let expected = vec![1.0, 2.0, 5.0, 8.0, 11.0];
        assert_eq!(sequence, expected)
    }
}
