use std::collections::BTreeMap;

// TODO, refactor this module.
// TODO try and remove all references to clone and copy.
// TODO update name of Term.
#[derive(Clone)]
pub struct Term<A, B = A>(Vec<A>, Vec<B>);

// TODO add proper documentation.
pub trait TermStructure<X, Y = X>: Iterator<Item = (X, Y)>
where
    Self: Sized,
    X: Copy + Default + PartialEq,
    Y: Copy + Default + PartialEq,
{
    /// Construct new instance of Self.
    fn new(x: &[X], y: &[Y]) -> Self;

    /// Construct new instance of Self,
    /// ensuring 'x' and 'y' are of equal length
    /// through the use of padding.
    fn with_padding(x: &[X], y: &[Y]) -> Self;

    /// Return 'x' value associated with Self.
    fn x(&self) -> Vec<X>;

    /// Return 'y' value associated with Self.
    fn y(&self) -> Vec<Y>;

    /// Update 'x' value using a closure.
    fn map_x<A>(&mut self, closure: A) -> &mut Self
    where
        A: Fn(&X) -> X;

    /// Update 'x' value using a closure.
    fn map_y<B>(&mut self, closure: B) -> &mut Self
    where
        B: Fn(&Y) -> Y;

    /// Return 'x' and 'y' as a tuple.
    fn unpack(&self) -> (Vec<X>, Vec<Y>) {
        (self.x(), self.y())
    }

    /// Increment each non-default 'y' element with a constant size.
    fn shift_y<C>(&mut self, size: C) -> &mut Self
    where
        C: Copy,
        Y: std::ops::Add<C, Output = Y>,
    {
        self.map_y(|a| if a != &Y::default() { *a + size } else { *a })
    }

    /// Update 'y' values of 'self' with 'other'.
    fn left_join(&mut self, other: Self) -> Self;
}

impl<X, Y> Iterator for Term<X, Y> {
    type Item = (X, Y);

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() || self.1.is_empty() {
            None
        } else {
            Some((self.0.remove(0), self.1.remove(0)))
        }
    }
}

impl<X> From<Vec<X>> for Term<X>
where
    X: Copy + Default + PartialEq,
    Term<X>: TermStructure<X>,
{
    fn from(value: Vec<X>) -> Self {
        let x = value[..value.len() - 1].to_vec();
        let y = value[1..].to_vec();
        Self::new(&x, &y)
    }
}

impl<X> From<Term<X>> for Vec<X>
where
    X: Copy + Default + PartialEq,
    Term<X>: TermStructure<X>,
{
    fn from(value: Term<X>) -> Self {
        let mut x = value.x();
        let y = value.y();
        x.push(y[y.len() - 1]);
        x
    }
}

impl<A, B, C, D> From<BTreeMap<A, B>> for Term<C, D>
where
    A: Copy + Into<C>,
    B: Copy + Into<D>,
{
    fn from(value: BTreeMap<A, B>) -> Self {
        let x = value.keys().copied().map(Into::into).collect();
        let y = value.values().copied().map(Into::into).collect();
        Self(x, y)
    }
}

impl<X, Y> TermStructure<X, Y> for Term<X, Y>
where
    X: Copy + Default + PartialEq,
    Y: Copy + Default + PartialEq,
{
    fn new(x: &[X], y: &[Y]) -> Self {
        assert_eq!(x.len(), y.len());
        Self(x.to_vec(), y.to_vec())
    }

    fn with_padding(x: &[X], y: &[Y]) -> Self {
        let x_len = x.len();
        let y_len = y.len();
        let mut x_padded = vec![X::default(); x_len.max(y_len) - x_len];
        let mut y_padded = vec![Y::default(); x_len.max(y_len) - y_len];

        x_padded.extend(x.to_vec());
        y_padded.extend(y.to_vec());
        Self(x_padded, y_padded)
    }

    fn x(&self) -> Vec<X> {
        self.0.to_vec()
    }

    fn y(&self) -> Vec<Y> {
        self.1.to_vec()
    }

    fn map_x<A>(&mut self, closure: A) -> &mut Self
    where
        A: Fn(&X) -> X,
    {
        self.0 = self.0.iter().map(closure).collect();
        self
    }

    fn map_y<B>(&mut self, closure: B) -> &mut Self
    where
        B: Fn(&Y) -> Y,
    {
        self.1 = self.1.iter().map(closure).collect();
        self
    }

    fn left_join(&mut self, other: Self) -> Self {
        let x = self.x();
        let mut y = self.y();
        for (i, x_val_self) in x.iter().enumerate() {
            if let Some(pos) = other
                .x()
                .into_iter()
                .position(|x_val_other| *x_val_self == x_val_other)
            {
                y[i] = other.y()[pos];
            }
        }
        Self::new(&x, &y)
    }
}
