/// Future value derived using simple compounding.
pub fn simple_fv(pv: f64, r: f64, n: f64) -> f64 {
    pv * (1.0 + r * n)
}

/// Present value derived using simple compounding.
pub fn simple_pv(fv: f64, r: f64, n: f64) -> f64 {
    fv * (1.0 + r * n).powf(-1.0)
}

/// Future value derived using discrete compounding.
pub fn discrete_fv(pv: f64, r: f64, m: f64, n: f64) -> f64 {
    pv * (1.0 + r / m).powf(m * n)
}

/// Present value derived using discrete compounding.
pub fn discrete_pv(fv: f64, r: f64, m: f64, n: f64) -> f64 {
    fv * (1.0 + r / m).powf(-m * n)
}

/// Future value derived using continious compounding.
pub fn continous_fv(pv: f64, r: f64, n: f64) -> f64 {
    pv * std::f64::consts::E.powf(r * n)
}

/// Present value derived using continious compounding.
pub fn continous_pv(fv: f64, r: f64, n: f64) -> f64 {
    fv * std::f64::consts::E.powf(-r * n)
}

// Compounding frequency conversion.
// FIXME logic is currently incorrect.
pub fn simple_to_discrete(r: f64, m: f64) -> f64 {
    ((1.0 + r).powf(1.0 / m) - 1.0) * m
}

pub fn simple_to_continous(r: f64) -> f64 {
    (1.0 + r).ln()
}

pub fn discrete_to_simple(r: f64, m: f64) -> f64 {
    (1.0 + r / m).powf(m) - 1.0
}

pub fn discrete_to_discrete(r: f64, from_freq: f64, to_freq: f64) -> f64 {
    to_freq * ((1.0 + r / from_freq).powf(from_freq / to_freq) - 1.0)
}

pub fn discrete_to_continous(r: f64, m: f64) -> f64 {
    m * (1.0 + r / m).ln()
}

pub fn continious_to_simple(r: f64) -> f64 {
    std::f64::consts::E.powf(r) - 1.0
}

pub fn continious_to_discrete(r: f64, m: f64) -> f64 {
    m * (std::f64::consts::E.powf(r / m) - 1.0)
}
