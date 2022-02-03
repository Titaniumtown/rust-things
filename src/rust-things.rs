extern crate rayon;
extern crate primal;
extern crate indicatif;

// integers
extern crate num_bigint;
extern crate rand;
extern crate num_traits;
extern crate rug;

#[macro_use]
extern crate lazy_static;

mod threecubes;
mod goldenratio;
mod pi;
mod primes;

fn main() {
    #[allow(unused_mut)]
    // let mut output: String; // Output variable

    // sum of three cubes stuff:
    /*
    #[allow(unused_mut)]
    let mut smallestcube;

    // let output = threecubes::basicthreecubes(200, 69);
    // let output = threecubes::threadedthreecubes(200, 69);
    println!("List of cubes: {:?}", output);
    smallestcube = threecubes::getsmallestcube(output);
    println!("Smallest cube: {:?}", smallestcube);
    // */

    // goldenratio:
    // println!("{:?}", goldenratio::goldenratio(20));

    // pi:
    // /*
    // println!("{:?}", pi::pimultithreaded(10000000000));
    // println!("{:?}", pi::pisinglethreaded(1000000000));
    // */

    // primes:
    let prime_opts = [0].to_vec();
    // println!("{:?}", primes::mersenne_prime_parallel(0, 150000, prime_opts));
    // println!("{:?}", primes::mersenne_prime_parallel(0, 20000, prime_opts));
    println!("{:?}", primes::mersenne_prime_parallel(0, 10000, prime_opts));
    // println!("{:?}", primes::mersenne_prime_parallel(0, 100, prime_opts));
}