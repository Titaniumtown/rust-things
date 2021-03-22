use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

// Single threaded implementation to find Mersenne Primes
pub fn mersenne_prime_basic(n: i32) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::new();
    let numrange = 1..n;
    for x in numrange {
        if is_mersenne_i32(x) {
            println!("Found: 2^{}-1 ", x);
            output.push(x);
        }
    }
    return output;
}

fn is_mersenne_i32(n: i32) -> bool {
    let flag: bool = lucas_lehmer::is_mersenne(n as usize);
    if flag {println!("Found: 2^{}-1 ", n)};
    return flag;
}

// Multi threaded implementation to find Mersenne Primes
pub fn mersenne_prime_parallel(n: i32) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::new();
    let numrange = 1..n;
    output = numrange.into_par_iter().filter(|&x| is_mersenne_i32(x)).collect();
    return output;
}