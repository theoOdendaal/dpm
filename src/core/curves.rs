use std::collections::BTreeMap;
pub struct Curve(BTreeMap<u32, f64>);

// What are the trait requirements for a BTreeMap?

//  --- Implementations: Standard library traits ---
impl From<BTreeMap<u32, f64>> for Curve {
    fn from(value: BTreeMap<u32, f64>) -> Self {
        Curve(value)
    }
}

impl Curve {
    fn x(&self) -> Vec<f64> {
        let mut x: Vec<f64> = self.0.keys().map(|a| *a as f64).collect();
        x.shrink_to_fit();
        x
    }

    fn y(&self) -> Vec<f64> {
        let mut y: Vec<f64> = self.0.values().copied().collect();
        y.shrink_to_fit();
        y
    }
}

impl From<Curve> for (Vec<f64>, Vec<f64>) {
    fn from(value: Curve) -> Self {
        (value.x(), value.y())
    }
}
