pub fn compute_approximation(data: &[f64]) -> Vec<[f64; 2]> {
    let mut result = Vec::new();
    let n = data.len();

    if n < 3 {
        return result;
    }

    for i in 1..n - 1 {
        let x = i as f64;
        let f0 = data[i];
        let fp = (data[i + 1] - data[i - 1]) / 2.0;
        let fpp = (data[i + 1] - 2.0 * data[i] + data[i - 1]);
        let approx = f0 + fp * 1.0 + (fpp / 2.0);
        result.push([x + 1.0, approx]);
    }

    result
}
