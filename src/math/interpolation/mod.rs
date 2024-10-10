// TODO if xp = 0, return 1.0.
// TODO complete this module.
// TODO implement unit tests.
// TODO use WolframAlpha to create unit tests.

// To solve for the coefficients of polynomial functions, you generally use systems of linear equations,
// especially when you have a set of points that the polynomial must pass through.
// For a polynomial of degree 𝑛, you need 𝑛 + 1 points to determine the coefficients uniquely.
// Example:
// - Lagrange Interpolation
// - Newton Interpolation

// Lagrange interpolation provides a way to fit a polynomial to a set of data points by leveraging basis
// polynomials that are constructed to pass through each point exactly. It’s particularly useful for
// constructing polynomials from a small number of data points.

// Nelson-Siegel.
// Fit Nelson-Siegel similar to how a neural network parameters are set.

// Research 'Lagrange Interpolation'.

//  --- Errors

//  --- Types
type QuadraticPoints<'a> = (&'a f64, &'a f64, &'a f64);
type CubicPoints<'a> = (&'a f64, &'a f64, &'a f64, &'a f64);

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
pub struct CubicHermite;
pub struct LogLinear;
pub struct Quadratic;
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
            Self::CubicHermite => CubicHermite.interpolate(x, y, xp),
            Self::LogLinear => LogLinear.interpolate(x, y, xp),
            Self::Quadratic => Quadratic.interpolate(x, y, xp),
            Self::Exponential => Exponential.interpolate(x, y, xp),
            Self::Akima => todo!(),
        }
    }
}

impl Interpolate<Vec<f64>, f64> for Linear {
    fn interpolate(&self, x: &Vec<f64>, y: &Vec<f64>, xp: &f64) -> f64 {
        // TODO is the below condition appropriate?
        if xp <= &0.0 {
            if xp < &0.0 {
                return 0.0;
            } else {
                return 1.0;
            }
        };

        let index = partition_index(x, xp);
        let (x1, x2) = (x[index], x[index + 1]);
        let (y1, y2) = (y[index], y[index + 1]);

        ((xp - x1) / (x2 - x1)) * (y2 - y1) + y1
    }
}

// TODO is this really CubicHermite? Isn't 3 + 1 points required?
// TODO look at this guidance: https://www.calculatorsoup.com/calculators/algebra/cubicequation.php
impl Interpolate<Vec<f64>, f64> for CubicHermite {
    fn interpolate(&self, x: &Vec<f64>, y: &Vec<f64>, xp: &f64) -> f64 {
        if xp <= &0.0 {
            if xp < &0.0 {
                return 0.0;
            } else {
                return 1.0;
            }
        };

        let index = partition_index(x, xp).max(1);

        let (x1, x2, x3) = (&x[index - 1], &x[index], &x[index + 1]);
        let (y1, y2, y3) = (&y[index - 1], &y[index], &y[index + 1]);

        let t = (xp - x2) / (x3 - x2);
        let m0 = (y2 - y1) / (x2 - x1);
        //let m1 = (y3 - y2) / (x3 - x2);
        let m1 = (y3 - y1) / (x3 - x1); // Smooth curve by calculating the curve, by using an average.

        let h1 = (2.0 * t.powf(3.0) - 3.0 * t.powf(2.0) + 1.0) * y1;
        let h2 = (-2.0 * t.powf(3.0) + 3.0 * t.powf(2.0)) * y2;
        let h3 = (t.powf(3.0) - 2.0 * t.powf(2.0) + t) * m0 * (x2 - x1);
        let h4 = (t.powf(3.0) - t.powf(2.0)) * m1 * (x2 - x1);

        h1 + h2 + h3 + h4
    }
}

impl Interpolate<Vec<f64>, f64> for LogLinear {
    fn interpolate(&self, x: &Vec<f64>, y: &Vec<f64>, xp: &f64) -> f64 {
        if xp <= &0.0 {
            if xp < &0.0 {
                return 0.0;
            } else {
                return 1.0;
            }
        };

        let index = partition_index(x, xp);
        let (x1, x2) = (x[index], x[index + 1]);
        let (y1, y2) = (y[index], y[index + 1]);

        std::f64::consts::E
            .powf(((xp - x1) / (x2 - x1)) * y2.ln() + ((x2 - xp) / (x2 - x1)) * y1.ln())
    }
}

impl Interpolate<Vec<f64>, f64> for Quadratic {
    fn interpolate(&self, x: &Vec<f64>, y: &Vec<f64>, xp: &f64) -> f64 {
        if xp <= &0.0 {
            if xp < &0.0 {
                return 0.0;
            } else {
                return 1.0;
            }
        };

        let index = partition_index(x, xp);

        // TODO, is this the best wat to index? Should i not .max(1) then index and start at -1?
        let x_values = (&x[index], &x[index + 1], &x[index + 2]);
        let y_values = (&y[index], &y[index + 1], &y[index + 2]);

        let (a, b, c) = quadratic_coefficients(x_values, y_values);

        a * (xp).powf(2.0) + b * (xp) + c
    }
}

impl Interpolate<Vec<f64>, f64> for Exponential {
    fn interpolate(&self, x: &Vec<f64>, y: &Vec<f64>, xp: &f64) -> f64 {
        if xp <= &0.0 {
            if xp < &0.0 {
                return 0.0;
            } else {
                return 1.0;
            }
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

fn quadratic_coefficients(x: QuadraticPoints, y: QuadraticPoints) -> (f64, f64, f64) {
    let denominator = (x.0 - x.1) * (x.0 - x.2) * (x.1 - x.2);

    let a1 = x.2 * (y.1 - y.0);
    let a2 = x.1 * (y.0 - y.2);
    let a3 = x.0 * (y.2 - y.1);
    let a = (a1 + a2 + a3) / denominator;

    let b1 = x.2.powf(2.0) * (y.0 - y.1);
    let b2 = x.1.powf(2.0) * (y.2 - y.0);
    let b3 = x.0.powf(2.0) * (y.1 - y.2);
    let b = (b1 + b2 + b3) / denominator;

    let c1 = y.0 * x.1 * x.2 / ((x.0 - x.1) * (x.0 - x.2));
    let c2 = y.1 * x.0 * x.2 / ((x.1 - x.0) * (x.1 - x.2));
    let c3 = y.2 * x.0 * x.1 / ((x.2 - x.0) * (x.2 - x.1));
    let c = c1 + c2 + c3;

    (a, b, c)
}

fn cubic_coefficients(x: CubicPoints, y: CubicPoints) -> (f64, f64, f64, f64) {
    todo!()
}

/*

fn cubic_h1(t: &f64) -> f64 {
    2.0 * t.powf(3.0) - 3.0 * t.powf(2.0) + 1.0
}

fn cubic_h2(t: &f64) -> f64 {
    -2.0 * t.powf(3.0) + 3.0 * t.powf(2.0)
}

fn cubic_h3(t: &f64) -> f64 {
    t.powf(3.0) - 2.0 * t.powf(2.0) + t
}

fn cubic_h4(t: &f64) -> f64 {
    t.powf(3.0) - t.powf(2.0)
}
*/

//  --- Unit tests
#[cfg(test)]
mod test_interpolation {
    use crate::assert_approx_eq;

    use super::*;

    #[test]
    fn test_quadratic_coefficients() {
        let x = (&0.1, &0.2, &0.5);
        let y = (&1.2, &1.7, &2.1);
        let (a, b, c) = quadratic_coefficients(x, y);
        // Obtain from: WolframAlpha
        assert_approx_eq!(a, -9.166667);
        assert_approx_eq!(b, 7.75);
        assert_approx_eq!(c, 0.516667);
    }
}
