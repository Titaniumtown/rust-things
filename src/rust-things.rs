extern crate rayon;
extern crate primal;
extern crate indicatif;

// integers
extern crate ramp;
extern crate num_bigint;
extern crate rand;
extern crate rug;

mod threecubes;
mod goldenratio;
mod pi;
mod primes;
mod newton_aprox;

 // todo - implement cmd args

fn main() {
    // sum of three cubes stuff:
    /*
    let mut output: String; // Output variable
    let mut smallestcube;

    let output = threecubes::basicthreecubes(200, 69);
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
    // println!("{:?}", pi::pimultithreaded_rug(10000, 128));

    // primes:
    // let prime_opts = [0].to_vec();
    // println!("{:?}", primes::mersenne_prime_parallel(0, 150000, prime_opts));
    // println!("{:?}", primes::mersenne_prime_parallel(0, 20000, prime_opts));
    // println!("{:?}", primes::prime_finder_parallel(0, 20000));

    // newton aprox
    println!("{:?}", newton_aprox::test(10, 10, 64));
}