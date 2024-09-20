//  --- Trait definitions ---
pub trait Interpolate<A, B> {
    fn interpolate(x: &A, y: &A, xp: &B) -> B;
}

//  --- Structs ---
pub struct Linear;
pub struct LogLinear;

//  --- Trait implementations ---
impl Interpolate<Vec<f64>, f64> for Linear {
    fn interpolate(x: &Vec<f64>, y: &Vec<f64>, xp: &f64) -> f64 {
        let index = get_index(x, xp);
        let (x1, x2) = (x[index], x[index + 1]);
        let (y1, y2) = (y[index], y[index + 1]);

        ((xp - x1) / (x2 - x1)) * (y2 - y1) + y1
    }
}

impl Interpolate<Vec<f64>, f64> for LogLinear {
    fn interpolate(x: &Vec<f64>, y: &Vec<f64>, xp: &f64) -> f64 {
        let index = get_index(x, xp);
        let (x1, x2) = (x[index], x[index + 1]);
        let (y1, y2) = (y[index], y[index + 1]);

        std::f64::consts::E
            .powf(((xp - x1) / (x2 - x1)) * y2.ln() + ((x2 - xp) / (x2 - x1)) * y1.ln())
    }
}

//  --- Helper functions ---
/// Return the index of the biggest 'x' element, smaller than or equal to "xp".
fn get_index(x: &[f64], xp: &f64) -> usize {
    let values_smaller: Vec<&f64> = x.iter().filter(|element| *element <= xp).collect();

    // Convert 'length' into 'index'.
    let index_count = values_smaller.len() - 1;

    // Clips the value at zero.
    let index_count = index_count.max(0);

    // 'x'.len() - 1 would represent the last index
    // within the collection. However, there should

    index_count.min(x.len() - 2)
}
