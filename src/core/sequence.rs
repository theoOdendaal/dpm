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
    A: Copy + std::cmp::Ord + std::ops::Sub<B, Output = A>,
    B: Copy,
{
    fn seq(lower: A, upper: A, step: B) -> Vec<A> {
        let min_value = lower.min(upper);
        let mut max_value = upper.max(lower);

        let mut sequence: Vec<A> = Vec::new();

        while max_value > min_value {
            sequence.push(max_value);
            max_value = max_value - step;
        }

        sequence.push(min_value);
        sequence.reverse();
        sequence
    }
}
