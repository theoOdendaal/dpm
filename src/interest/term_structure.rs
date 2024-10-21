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

// TODO, refactor this module.

// TODO try and remove all references to clone and copy.
// TODO create more Error variants.
// TODO impl From<> for Error for more instances.

// TODO should the CurveParameters not be a struct with concrete implementations?
// Make the From<BTreeMap> conversion generic.

//  --- Errors

pub struct TermStructure<A, B = A> {
    index: usize,
    x: Vec<A>,
    y: Vec<B>,
}

impl<A, B, C, D> From<BTreeMap<A, B>> for TermStructure<C, D>
where
    A: Clone + Into<C>,
    B: Clone + Into<D>,
{
    fn from(value: BTreeMap<A, B>) -> Self {
        let x = value.keys().cloned().map(Into::into).collect();
        let y = value.values().cloned().map(Into::into).collect();
        Self { index: 0, x, y }
    }
}

impl<A: PartialEq, B: Clone> TermStructure<A, B> {
    pub fn update_with(&mut self, other: Self) {
        for (i, x_val_self) in self.x.iter().enumerate() {
            if let Some(pos) = other
                .x
                .iter()
                .position(|x_val_other| x_val_self == x_val_other)
            {
                // If x is found in other, update y in self with the corresponding y from other
                self.y[i] = other.y[pos].clone();
            }
        }
    }
}

impl<A, B> TermStructure<A, B>
where
    A: Clone + Default,
    B: Clone + Default,
{
    pub fn new(x: &[A], y: &[B]) -> Self {
        Self {
            index: 0,
            x: x.to_vec(),
            y: y.to_vec(),
        }
    }

    pub fn new_from_map_x<F, C>(&self, closure: F) -> TermStructure<C, B>
    where
        F: Fn(&A) -> C,
    {
        let x = self.get_x().iter().map(closure).collect::<Vec<C>>();
        let y = self.get_y();
        TermStructure { index: 0, x, y }
    }

    pub fn new_from_map_y<F, C>(&self, closure: F) -> TermStructure<A, C>
    where
        F: Fn(&B) -> C,
    {
        let x = self.get_x();
        let y = self.get_y().iter().map(closure).collect::<Vec<C>>();
        TermStructure { index: 0, x, y }
    }

    pub fn new_with_left_pad(x: &[A], y: &[B]) -> Self {
        let x_len = x.len();
        let y_len = y.len();
        let mut x_padded = vec![A::default(); x_len.max(y_len) - x_len];
        let mut y_padded = vec![B::default(); x_len.max(y_len) - y_len];

        x_padded.extend(x.to_vec());
        y_padded.extend(y.to_vec());
        Self {
            index: 0,
            x: x_padded,
            y: y_padded,
        }
    }

    /// Return 'key' field.
    pub fn get_x(&self) -> Vec<A> {
        self.x.to_vec()
    }

    /// Return 'value' field
    pub fn get_y(&self) -> Vec<B> {
        self.y.to_vec()
    }

    /// Returns a tuple containing the 'key' and 'value' field.
    pub fn unpack(&self) -> (Vec<A>, Vec<B>) {
        (self.get_x(), self.get_y())
    }

    /// Map 'key' field using a closure.
    pub fn map_x<F>(&mut self, closure: F) -> &mut Self
    where
        F: Fn(&A) -> A,
    {
        self.x = self.get_x().iter().map(closure).collect::<Vec<A>>();
        self
    }

    /// Map 'value' field using a closure.
    pub fn map_y<F>(&mut self, closure: F) -> &mut Self
    where
        F: Fn(&B) -> B,
    {
        self.y = self.get_y().iter().map(closure).collect::<Vec<B>>();
        self
    }
}

impl<A, B> Iterator for TermStructure<A, B>
where
    A: Clone,
    B: Clone,
{
    type Item = (A, B);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.x.len() && self.index < self.y.len() {
            let item = (self.x[self.index].clone(), self.y[self.index].clone());
            self.index += 1;
            Some(item)
        } else {
            self.index = 0;
            None
        }
    }
}
