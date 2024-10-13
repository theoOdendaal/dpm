// 1.
// Distinguish between 'static' and 'market data'.
// 'Static' is contractual data, where 'market data' is curves, rates etc.
// Each of the above should be handled separately.

// 2.
// Refer to ISDA definitions for terminology/'jargon'.

// 3.
// All functions should take Vec<f64> as self.

use std::collections::BTreeMap;

use dpm::interest::term_structure::{CurveParameters, TermStructure};

fn main() {
    let path = "src/resources/curves";
    let dir = format!("{}/zar_disc_csa.txt", path);
    let contents = std::fs::read_to_string(dir).unwrap();
    let curve: BTreeMap<u32, f64> = serde_json::from_str(&contents).unwrap();
    let curve: CurveParameters<f64> = curve.into();

    let xp: Vec<f64> = (1..100).map(|x| x as f64 / 365.0).collect();
    let (x, y) = curve.unpack_with_map_x(|a| a / 365.0);

    // mod.rs
    /*
    let epochs = 1_000;
    let lr = 1e-11;
    let coef = get_coefficients(&x, &y, 20, epochs, lr);
    let res = predict(&xp, &coef);
    println!("{:?}", res);
    */

    // poly.rs
    /*
    let (a, b, c, d) = solve_for_cubic_coefficients(&x, &y);

    let interpolated_points: Vec<f64> = xp
        .into_iter()
        .map(|xi| cubic_function(&a, &b, &c, &d, &xi))
        .collect();

    println!("{:?}", interpolated_points);
    */
}
