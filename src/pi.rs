use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rug::Float;

#[allow(dead_code)]
pub fn pisinglethreaded(n: i64) -> f64 {
    let mut total: f64 = 0.0;
    for i in 1..n {
        total += 1.0/(f64::powi(i as f64, 2));
    }
    (6.0*total).sqrt()
}

#[allow(dead_code)]
pub fn pimultithreaded(n: i64) -> f64 {
    let total: f64 = (1..n).into_par_iter().map(|i| (1.0/(f64::powi(i as f64, 2)))).sum();
    (6.0*total).sqrt()
}

#[allow(dead_code)]
pub fn pimultithreaded_rug(n: i32, precision: u32) -> Float {
    let total: Vec<Float> = (1..n).into_par_iter().map(|i| {
        let exp_tmp: Float = Float::with_val(precision, Float::i_pow_u(i, 2)); // todo - figure out why `with_val` needs i32 instead of i64 and find solution
        1.0/exp_tmp
    }).collect();
    let mut sum_tmp: Float = Float::with_val(precision, 0); 
    for ele in total.iter() {
        sum_tmp += ele
    }
    let sum: Float = sum_tmp;

    let total_sum: Float = 6*sum;
    total_sum.sqrt()
}