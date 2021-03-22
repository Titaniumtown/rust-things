use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use ramp::Int;
use num_integer::Integer;

// Single threaded implementation to find Mersenne Primes
#[allow(dead_code)]
pub fn mersenne_prime_basic(start: i32, plus: i32) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::new();
    let numrange = start..(start+plus);
    for x in numrange {
        if is_mersenne_i32(x) {
            println!("Found: {}", x);
            output.push(x);
        }
    }
    return output;
}

pub fn is_mersenne(prime: usize) -> bool {
    if prime == 2 {return true};

    if !primal::is_prime(prime as u64) || prime.is_even() {
        return false;
    }

    return check_mersenne(prime);
}

fn check_mersenne(prime: usize) -> bool {
    let mersenne: Int = Int::from(Int::from(2).pow(prime) - 1);
    let mut s: Int = Int::from(4);

    for _i in 2..prime {
        s = (&s * &s - 2) % &mersenne;
    }

    s == 0
}


fn is_mersenne_i32(n: i32) -> bool {
    let flag: bool = is_mersenne(n as usize);
    if flag {println!("Found: {}", n)};
    return flag;
}

// Multi threaded implementation to find Mersenne Primes
#[allow(dead_code)]
pub fn mersenne_prime_parallel(start: i32, plus: i32) -> Vec<i32> {
    let numrange = start..(start+plus);
    let output: Vec<i32> = numrange.into_par_iter()
        .filter(|&x| is_mersenne_i32(x)).collect();
    return output;
}