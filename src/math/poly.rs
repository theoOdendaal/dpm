//https://www.kindsonthegenius.com/machine-learning-101-polynomial-curve-fitting/

pub fn predict(x: &[f64], coefficients: &[f64]) -> Vec<f64> {
    x.iter()
        .map(|xi| {
            coefficients
                .iter()
                .enumerate()
                .map(|(i, c)| c * xi.powi(i as i32))
                .sum::<f64>()
        })
        .collect::<Vec<f64>>()
}

pub fn loss(x: &[f64], y: &[f64], coefficients: &[f64]) -> f64 {
    let y_pred = predict(x, coefficients);
    let square_error: f64 = y
        .iter()
        .zip(y_pred.iter())
        .map(|(a, b)| (a - b).powi(2))
        .sum();
    square_error / x.len() as f64
}

pub fn gradients(x: &[f64], y: &[f64], coefficients: &[f64]) -> Vec<f64> {
    let y_pred = predict(x, coefficients);
    let mut grad = vec![0.0; coefficients.len()];

    for (i, xi) in x.iter().enumerate() {
        let error = y_pred[i] - y[i];
        for (j, g) in grad.iter_mut().enumerate() {
            *g += (2.0 * error * xi.powi(j as i32)).clamp(-1.0, 1.0);
        }
    }
    grad.iter().map(|g| g / x.len() as f64).collect()
}

pub fn get_coefficients(x: &[f64], y: &[f64], degrees: usize, epochs: usize, lr: f64) -> Vec<f64> {
    let mut coefficients: Vec<f64> = vec![0.0; degrees + 1];

    for _ in 0..epochs {
        let current_loss = loss(x, y, &coefficients);
        if current_loss.abs() < 1e-8 {
            return coefficients;
        }

        let formatted_coefficients: Vec<String> =
            coefficients.iter().map(|c| format!("{:.6}", c)).collect();
        println!("{:?} - Loss: {:.6}", formatted_coefficients, current_loss);

        let grads = gradients(x, y, &coefficients);

        for (i, g) in grads.iter().enumerate() {
            coefficients[i] -= lr * g;
        }
    }

    coefficients
}
