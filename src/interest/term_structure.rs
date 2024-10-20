use std::collections::BTreeMap;

/*
    use chrono::NaiveDate;
    use dpm::core::curves::{Curve, CurveParameters};

    let x = vec!["2023-12-31", "2023-12-31"];
    let y = vec![0.06, 0.07];
    let curve = CurveParameters::new(&x, &y);
    if let Ok(trans_res) = curve.try_map_x(|a| NaiveDate::parse_from_str(a, "%Y-%m-%d")) {
        let curve = CurveParameters::new(&trans_res, &y);

        println!("{:?}", curve);
    }

*/

// TODO try and remove all references to clone and copy.
// TODO create more Error variants.
// TODO impl From<> for Error for more instances.
// TODO implement IntoIterator and Iterator for any type that implements the TermStructure trait. Iterator should return a tuple (x, y).

//  --- Errors

#[derive(Debug)]
pub enum Error {
    ParseError(chrono::format::ParseError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseError(err) => write!(f, "{:?}", err),
        }
    }
}

impl std::error::Error for Error {}

impl From<chrono::format::ParseError> for Error {
    fn from(value: chrono::format::ParseError) -> Self {
        Self::ParseError(value)
    }
}

//  --- Structs

#[derive(Debug)]
pub struct CurveParameters<A, B = A> {
    x: Vec<A>,
    y: Vec<B>,
}

//  --- Traits

/// Requirement for a type to be classified as a term structure.
pub trait TermStructure<A, B = A> {
    /// Return 'key' field of Curve.
    fn get_x(&self) -> Vec<A>;

    /// Return 'value' field of Curve.
    fn get_y(&self) -> Vec<B>;

    /// Returns a tuple containing the 'key' and 'value' field.
    fn unpack(&self) -> (Vec<A>, Vec<B>) {
        (self.get_x(), self.get_y())
    }

    /// Map 'key' field using a closure.
    fn map_x<F>(&self, closure: F) -> Vec<A>
    where
        F: Fn(&A) -> A,
    {
        self.get_x().iter().map(closure).collect::<Vec<A>>()
    }

    /// Map 'value' field using a closure.
    fn map_y<F>(&self, closure: F) -> Vec<B>
    where
        F: Fn(&B) -> B,
    {
        self.get_y().iter().map(closure).collect::<Vec<B>>()
    }

    /// Returns a tuple containing the 'key' and 'value' field.
    /// Where the 'key' is mapped using a closure.
    fn unpack_with_map_x<F>(&self, closure: F) -> (Vec<A>, Vec<B>)
    where
        F: Fn(&A) -> A,
    {
        (self.map_x(closure), self.get_y())
    }

    /// Returns a tuple containing the 'key' and 'value' field.
    /// Where the 'value' is mapped using a closure.
    fn unpack_with_map_y<F>(&self, closure: F) -> (Vec<A>, Vec<B>)
    where
        F: Fn(&B) -> B,
    {
        (self.get_x(), self.map_y(closure))
    }

    /// Map 'key' field using a closure, returing Result.
    fn try_map_x<F, E, C>(&self, closure: F) -> Result<Vec<C>, E>
    where
        F: Fn(&A) -> Result<C, E>,
        Error: From<E>,
    {
        self.get_x()
            .iter()
            .map(closure)
            .collect::<Result<Vec<C>, E>>()
    }

    /// Map 'value' field using a closure, returing Result.
    fn try_map_y<F, E, C>(&self, closure: F) -> Result<Vec<C>, E>
    where
        F: Fn(&B) -> Result<C, E>,
        Error: From<E>,
    {
        self.get_y()
            .iter()
            .map(closure)
            .collect::<Result<Vec<C>, E>>()
    }

    /// Returns a tuple containing the 'key' and 'value' field.
    /// Where the 'key' is mapped using a closure, returing a result.
    fn unpack_with_try_map_x<F, E>(&self, closure: F) -> Result<(Vec<A>, Vec<B>), E>
    where
        F: Fn(&A) -> Result<A, E>,
        Error: From<E>,
    {
        Ok((self.try_map_x(closure)?, self.get_y()))
    }

    /// Returns a tuple containing the 'key' and 'value' field.
    /// Where the 'value' is mapped using a closure, returing a result.
    fn unpack_with_try_map_y<F, E>(&self, closure: F) -> Result<(Vec<A>, Vec<B>), E>
    where
        F: Fn(&B) -> Result<B, E>,
        Error: From<E>,
    {
        Ok((self.get_x(), self.try_map_y(closure)?))
    }
}

//  --- Trait implementations: Concrete

impl<A, B> From<BTreeMap<A, B>> for CurveParameters<f64>
where
    A: Copy + Into<f64>,
    B: Copy + Into<f64>,
{
    fn from(value: BTreeMap<A, B>) -> Self {
        let mut x: Vec<f64> = value.keys().copied().map(Into::into).collect();
        let mut y: Vec<f64> = value.values().copied().map(Into::into).collect();
        x.shrink_to_fit();
        y.shrink_to_fit();
        Self { x, y }
    }
}

impl<A, B> CurveParameters<A, B>
where
    A: Clone,
    B: Clone,
{
    pub fn new(x: &[A], y: &[B]) -> Self {
        assert_eq!(x.len(), y.len()); // TODO Add more robust error handling.

        Self {
            x: x.to_vec(),
            y: y.to_vec(),
        }
    }
}

impl<A, B> TermStructure<A, B> for CurveParameters<A, B>
where
    A: Clone,
    B: Clone,
{
    fn get_x(&self) -> Vec<A> {
        self.x.to_vec()
    }

    fn get_y(&self) -> Vec<B> {
        self.y.to_vec()
    }
}
