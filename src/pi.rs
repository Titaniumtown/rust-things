extern crate rayon;
// use rayon::prelude::*;
use pi::rayon::iter::IntoParallelIterator;
use pi::rayon::iter::ParallelIterator;

pub fn pisinglethreaded(n: i64) -> f64 {
    let mut total: f64 = 0 as f64;
    for i in 1..n {
        total += (1.0/(f64::powi((i as f64),2)));
    }
    return (6.0*total).sqrt();
}

pub fn pimultithreaded(n: i64) -> f64 {
    let mut total: f64 = 0 as f64;
    let stuff = (1..n).into_par_iter().map(|i| (1.0/(f64::powi((i as f64),2))));
    total = stuff.sum();
    return (6.0*total).sqrt();
}