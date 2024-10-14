/*
let path = "src/resources/curves";
    let dir = format!("{}/zar_disc_csa.txt", path);
    let contents = std::fs::read_to_string(dir).unwrap();
    let curve: BTreeMap<u32, f64> = serde_json::from_str(&contents).unwrap();
    let curve: CurveParameters<f64> = curve.into();

    let xp: Vec<f64> = (1..100).map(|x| x as f64 / 365.0).collect();
    let (x, y) = curve.unpack_with_map_x(|a| a / 365.0);

    let coef = solve_for_cubic_coefficients(&x, &y, 10);

    let interpolated_points: Vec<f64> = xp
        .into_iter()
        .map(|xi| polynominal_function(&coef, &xi))
        .collect();

    println!("{:?}", interpolated_points);
*/

pub fn polynominal_function(coefficients: &[f64], x: &f64) -> f64 {
    coefficients
        .iter()
        .enumerate()
        .map(|(i, c)| c * x.powi(i as i32))
        .sum::<f64>()
}

pub fn solve_for_cubic_coefficients(x: &[f64], y: &[f64], degree: usize) -> Vec<f64> {
    assert_eq!(x.len(), y.len());

    let mut coefficients = vec![0.0; degree + 1];

    let learning_rate = 1e-8;
    let epochs = 10_000;
    let n = x.len() as f64;

    for _ in 0..epochs {
        let mut gradients = vec![0.0; degree + 1];

        let mut total_error = 0.0;

        for (xi, yi) in x.iter().zip(y.iter()) {
            let predict = polynominal_function(&coefficients, xi);
            let error = yi - predict;
            let error_derivative = -2.0 * error;
            total_error += (yi - predict).powi(2);

            let weight = 1.0;

            for (i, c) in gradients.iter_mut().enumerate() {
                *c += weight * error_derivative * xi.powi(i as i32);
            }
        }

        let grad_clip_threshold = 1.0;
        for c in gradients.iter_mut() {
            *c = c.min(grad_clip_threshold).max(-grad_clip_threshold) * learning_rate / n;
        }

        for i in 0..coefficients.len() {
            coefficients[i] -= gradients[i];
        }

        println!("{:.6}", total_error);

        if total_error.abs() < 1e-8 {
            return coefficients.to_vec();
        }
    }

    coefficients.to_vec()
}
