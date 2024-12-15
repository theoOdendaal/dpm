#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

//! Sequence creation.

// TODO amend so the payment month and day can be specified !!!

//  --- Traits

/// Bound inclusive sequence.
pub trait Sequence<A, B, C> {
    fn seq(lower: A, upper: A, step: B) -> C;
}

//  --- Trait implementations: Blanket

impl<A, B> Sequence<A, B, Vec<A>> for A
where
    A: Copy + std::cmp::PartialOrd + std::ops::Sub<B, Output = A> + std::ops::Add<B, Output = A>,
    B: Copy,
{
    fn seq(lower: A, upper: A, step: B) -> Vec<A> {
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
        sequence
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

        let sequence = i32::seq(upper, lower, step);
        let expected = vec![1, 2, 5, 8, 11];
        assert_eq!(sequence, expected)
    }

    #[test]
    fn test_u32_sequence() {
        let lower: u32 = 1;
        let upper: u32 = 11;
        let step: u32 = 3;

        let sequence = u32::seq(upper, lower, step);
        let expected = vec![1, 2, 5, 8, 11];
        assert_eq!(sequence, expected)
    }

    #[test]
    fn test_f64_sequence() {
        let lower = 1.0;
        let upper = 11.0;
        let step = 3.0;

        let sequence = f64::seq(upper, lower, step);
        let expected = vec![1.0, 2.0, 5.0, 8.0, 11.0];
        assert_eq!(sequence, expected)
    }
}
