extern crate rayon;
extern crate primal;
extern crate ramp;
extern crate num_integer;
extern crate indicatif;

mod threecubes;
mod goldenratio;
mod pi;
mod primes;

use std::env::args; // todo - implement args

fn main() {
    #[allow(unused_mut)]
    // let mut output: string; // Output variable

    // sum of three cubes stuff:
    /*
    #[allow(unused_mut)]
    let mut smallestcube;

    output = threecubes::basicthreecubes(1000, 2005);
    println!("List of cubes: {:?}", output);
    smallestcube = threecubes::getsmallestcube(output);
    println!("Smallest cube: {:?}", smallestcube);
    */

    // goldenratio:
    // println!("{:?}", goldenratio::goldenratio(20));

    // pi:
    /*
    println!("{:?}", pi::pimultithreaded(1000000000));
    // println!("{:?}", pi::pisinglethreaded(1000000000));
    */

    // primes:
    let prime_opts = [1].to_vec();
    println!("{:?}", primes::mersenne_prime_parallel(0, 100000, prime_opts));
    // println!("{:?}", primes::mersenne_prime_parallel(0, 100000, prime_opts));
    // println!("{:?}", primes::prime_finder_parallel(0, 100000));    
}