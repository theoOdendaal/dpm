// Research Gaussian elimination.
// https://www.bing.com/videos/riverview/relatedvideo?q=gaussian+elimination&mid=4ED02AAAB6B975E7ADE54ED02AAAB6B975E7ADE5&FORM=VIRE

// 3rd degree.
// Si(x) = ai + bi(x - xi) + ci(x - xi)^2 + di(x - x1)^3
// S'i(x) = bi + 2.ci(x - xi) + 3.di(x - xi)^2
// S''i(x) = 2.ci + 6.di(x - xi)

// 2nd degree.
// Si(x) = ai + bi(x - x1) + ci(x - xi)^2
// S'i(x) = bi + 2.ci(x - xi)
// S''i(x) = 2.ci

// Conditions:
// 1. Si(xi) = yi
// 2. Si(x1+1) = yi+1
// 3. S'i(xi+1) = S'i+1(xi+1)
// 4. S''i(xi+1) = S''x1+1(x1+1)
// 5. S''0(x0) = 0 AND S''n-1(xn) = 0

fn polynomial(x: &f64, coefficients: &[f64]) -> f64 {
    coefficients
        .iter()
        .enumerate()
        .map(|(i, c)| x * c.powi(i as i32))
        .sum()
}
