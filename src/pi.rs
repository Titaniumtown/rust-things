use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[allow(dead_code)]
pub fn pisinglethreaded(n: i64) -> f64 {
    let mut total: f64 = 0.0;
    (1..n).for_each(|i| {
        total += 1.0/(f64::powi(i as f64, 2));
    });
    return (6.0*total).sqrt();
}

#[allow(dead_code)]
pub fn pimultithreaded(n: i64) -> f64 {
    let total: f64 = (1..n).into_par_iter().map(|i| (1.0/(f64::powi(i as f64, 2)))).sum();
    return (6.0*total).sqrt();
}
