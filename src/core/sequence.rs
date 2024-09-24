// Sequence creation.

/// Bound inclusive sequence.
pub trait Sequence<A, B, C> {
    fn seq(lower: A, upper: A, step: B) -> C;
}

// TODO impl !
pub trait LinearSequence<A, B> {
    fn lin_space(lower: A, upper: A, n: usize) -> B;
}

impl<A, B> Sequence<A, B, Vec<A>> for A
where
    A: Copy + std::cmp::Ord + std::ops::Sub<B, Output = A> + std::ops::Add<B, Output = A>,
    B: Copy,
{
    fn seq(lower: A, upper: A, step: B) -> Vec<A> {
        let min_value = lower.min(upper);
        let mut max_value = upper.max(lower);

        let mut sequence: Vec<A> = Vec::new();

        while max_value > min_value {
            sequence.push(max_value);

            max_value = max_value.max(min_value + step) - step;
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
}
