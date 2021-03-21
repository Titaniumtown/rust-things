pub fn pi(n: i64) -> f64 {
    let mut total: f64 = 0 as f64;
    for i in 1..n {
        total += (1.0/(f64::powi((i as f64),2)));
    }
    return (6.0*total).sqrt();
}