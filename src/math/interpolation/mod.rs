// TODO if xp = 0, return 1.0.
// TODO complete this module.

//  --- Errors

//  --- Enums
#[derive(Debug)]
pub enum InterpolationMethod {
    Linear,
    PiecewiseLinear,
    NelsonSiegel,
    NelsonSiegelSvensson,
    CubicSpline,  // Focuses on smoothness between intervals.
    CubicHermite, // Focuses on fitting both function and derivatives.
    LogLinear,
    Quadratic,
    Exponential,
    Akima,
}

impl Default for InterpolationMethod {
    fn default() -> Self {
        Self::LogLinear
    }
}

//  --- Structs
pub struct Linear;
pub struct LogLinear;
pub struct Exponential;

//  --- Traits
pub trait Interpolate<A, B> {
    fn interpolate(&self, x: &A, y: &A, xp: &B) -> B;
}

//  --- Trait implementations: Concrete

impl Interpolate<Vec<f64>, f64> for InterpolationMethod {
    fn interpolate(&self, x: &Vec<f64>, y: &Vec<f64>, xp: &f64) -> f64 {
        match self {
            Self::Linear => Linear.interpolate(x, y, xp),
            Self::PiecewiseLinear => todo!(),
            Self::NelsonSiegel => todo!(),
            Self::NelsonSiegelSvensson => todo!(),
            Self::CubicSpline => todo!(),
            Self::CubicHermite => todo!(),
            Self::LogLinear => LogLinear.interpolate(x, y, xp),
            Self::Quadratic => todo!(),
            Self::Exponential => Exponential.interpolate(x, y, xp),
            Self::Akima => todo!(),
        }
    }
}

impl Interpolate<Vec<f64>, f64> for Linear {
    fn interpolate(&self, x: &Vec<f64>, y: &Vec<f64>, xp: &f64) -> f64 {
        if xp == &0.0 {
            return 1.0;
        };

        let index = partition_index(x, xp);
        let (x1, x2) = (x[index], x[index + 1]);
        let (y1, y2) = (y[index], y[index + 1]);

        ((xp - x1) / (x2 - x1)) * (y2 - y1) + y1
    }
}

impl Interpolate<Vec<f64>, f64> for LogLinear {
    fn interpolate(&self, x: &Vec<f64>, y: &Vec<f64>, xp: &f64) -> f64 {
        if xp == &0.0 {
            return 1.0;
        };

        let index = partition_index(x, xp);
        let (x1, x2) = (x[index], x[index + 1]);
        let (y1, y2) = (y[index], y[index + 1]);

        std::f64::consts::E
            .powf(((xp - x1) / (x2 - x1)) * y2.ln() + ((x2 - xp) / (x2 - x1)) * y1.ln())
    }
}

impl Interpolate<Vec<f64>, f64> for Exponential {
    fn interpolate(&self, x: &Vec<f64>, y: &Vec<f64>, xp: &f64) -> f64 {
        if xp == &0.0 {
            return 1.0;
        };

        let index = partition_index(x, xp);
        let (x1, x2) = (x[index], x[index + 1]);
        let (y1, y2) = (y[index], y[index + 1]);

        y2.powf((xp - x1) / (x2 - x1)) * y1.powf((x2 - xp) / (x2 - x1))
    }
}

//  --- Trait implementations: Blanket
impl<A> Interpolate<Vec<f64>, Vec<f64>> for A
where
    A: Interpolate<Vec<f64>, f64>,
{
    fn interpolate(&self, x: &Vec<f64>, y: &Vec<f64>, xp: &Vec<f64>) -> Vec<f64> {
        xp.iter().map(|a| self.interpolate(x, y, a)).collect()
    }
}

//  --- Standalone functions
/// Return the index of the point that partitions 'x'. The index
/// is clipped at 0 and length less 2.
fn partition_index(x: &[f64], xp: &f64) -> usize {
    let values_smaller: Vec<&f64> = x.iter().filter(|element| *element <= xp).collect();

    // Convert 'length' into 'index'.
    let index_count = values_smaller.len().max(1) - 1;

    // Clips the value at zero.
    let index_count = index_count.max(0);

    // 'x'.len() - 1 would represent the last index
    // within the collection. However, there should
    // be left room for index + 1 (which is equivalent to len - 1);
    index_count.min(x.len() - 2)
}
