//! Sequence creation.

/// Bound inclusive sequence.
pub trait Sequence<A, B, C> {
    fn seq(lower: A, upper: A, step: B) -> C;
}

impl<A, B> Sequence<A, B, Vec<A>> for A
where
    A: Copy + std::cmp::PartialOrd + std::ops::Sub<B, Output = A> + std::ops::Add<B, Output = A>,
    B: Copy,
{
    fn seq(lower: A, upper: A, step: B) -> Vec<A> {
        //assert_eq!(lower + step, lower); // Checks that

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

#[cfg(test)]
mod test_frequency {

    use super::*;

    #[test]
    fn test_u32_sequence() {
        let lower = 1;
        let upper = 11;
        let step = 3;

        let sequence = i32::seq(upper, lower, step);
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
