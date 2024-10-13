// To be used for:
//  1. Type conversion
//  2. Determining polynomial coefficients.
//  3. Hazard rate model.
//  4. Solving for the hype derivative fixed rate.

pub trait NewtonRaphson<A>
where
    A: Fn(f64) -> f64,
{
    const MAX_ITER: usize = 1_000_000;
    const TOLERANCE: f64 = 1e-16;
    const STEP: f64 = 1e-8;

    fn solve(init: &f64, closure: A) -> f64 {
        let mut start = *init;
        let mut result_check = closure(start);
        let mut iterations = 0;

        while (1.0 - result_check).abs() > Self::TOLERANCE && iterations < Self::MAX_ITER {
            let derivative = ((closure(start + Self::STEP)) - result_check) / Self::STEP;

            start += (1.0 - result_check) / derivative;

            result_check = closure(start);
            iterations += 1;
        }
        println!("{:?}", iterations);
        start
    }
}

impl<A> NewtonRaphson<A> for f64 where A: Fn(f64) -> f64 {}

//  --- Old code.

#[derive(Default)]
pub enum SolvingMethod {
    #[default]
    GRGNonLinear,
    SimplexLP,
    Evolutionary,
}

pub trait SolveEquation<A, B, C, D>
where
    A: Fn(&B) -> C,
{
    /// Fn whose argument will be solved for.
    fn forward(var: &B, closure: &A) -> C {
        closure(var)
    }

    //fn backward(err: C, closure: D) -> C;
    fn backward(var: &B, closure: &D) -> C;

    //fn error

    /// Solves for the 'var' that would result in the 'forward' fn to produce a desired result.
    fn solve(f_closure: &A, b_closure: &D, target: &C) -> C;
}

pub struct Simple;

pub struct Discrete;

// TODO how to refactor the below?

impl SolveEquation<Box<dyn Fn(&f64) -> f64>, f64, f64, Box<dyn Fn() -> f64>> for Simple {
    fn forward(var: &f64, closure: &Box<dyn Fn(&f64) -> f64>) -> f64 {
        closure(var)
    }

    fn backward(var: &f64, closure: &Box<dyn Fn() -> f64>) -> f64 {
        closure()
    }

    fn solve(
        f_closure: &Box<dyn Fn(&f64) -> f64>,
        b_closure: &Box<dyn Fn() -> f64>,
        target: &f64,
    ) -> f64 {
        let mut init = 0.0;
        loop {
            let forward_value: f64 = Self::forward(&init, f_closure);
            let err = target - forward_value;
            if err.abs() < 1e-14 {
                return init;
            } else {
                init += err * Self::backward(&init, b_closure) * 0.01;
            }
        }
    }
}

impl SolveEquation<Box<dyn Fn(&f64) -> f64>, f64, f64, Box<dyn Fn(&f64) -> f64>> for Discrete {
    fn forward(var: &f64, closure: &Box<dyn Fn(&f64) -> f64>) -> f64 {
        closure(var)
    }

    fn backward(var: &f64, closure: &Box<dyn Fn(&f64) -> f64>) -> f64 {
        closure(var)
    }

    fn solve(
        f_closure: &Box<dyn Fn(&f64) -> f64>,
        b_closure: &Box<dyn Fn(&f64) -> f64>,
        target: &f64,
    ) -> f64 {
        let mut init = 0.0;
        loop {
            let forward_value: f64 = Self::forward(&init, f_closure);
            let err = target - forward_value;
            if err.abs() < 1e-10 {
                return init;
            } else {
                init += err * Self::backward(&init, b_closure) * 0.01;
            }
        }
    }
}
