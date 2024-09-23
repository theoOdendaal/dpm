use std::collections::BTreeMap;
pub struct Curve(BTreeMap<u32, f64>);

//  --- Implementations: Standard library traits ---
impl From<BTreeMap<u32, f64>> for Curve {
    fn from(value: BTreeMap<u32, f64>) -> Self {
        Curve(value)
    }
}

impl From<Curve> for (Vec<f64>, Vec<f64>) {
    fn from(value: Curve) -> Self {
        let mut x = Vec::new();
        let mut y = Vec::new();
        for (a, b) in value.0.iter() {
            x.push(*a as f64);
            y.push(*b);
        }

        (x, y)
    }
}
