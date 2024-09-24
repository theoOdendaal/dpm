use std::collections::BTreeMap;

// What are the trait requirements for a BTreeMap?

//  --- Trait definitions ---

pub trait Curve<T> {
    fn x(&self) -> Vec<T>;

    fn y(&self) -> Vec<T>;
}

//  --- Implementations: Blanket ---

impl<A, B> Curve<f64> for BTreeMap<A, B>
where
    A: Copy + Into<f64>,
    B: Copy + Into<f64>,
    Vec<f64>: FromIterator<A> + FromIterator<B>,
{
    fn x(&self) -> Vec<f64> {
        let mut x: Vec<f64> = self.keys().copied().map(Into::into).collect();
        x.shrink_to_fit();
        x
    }

    fn y(&self) -> Vec<f64> {
        let mut y: Vec<f64> = self.values().copied().map(Into::into).collect();
        y.shrink_to_fit();
        y
    }
}

//  --- Implementations: Custom traits ---
