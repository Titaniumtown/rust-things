extern crate rayon;
extern crate ramp;
extern crate primal;
extern crate num_integer;

mod threecubes;
mod goldenratio;
mod pi;
mod primes;

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
    // println!("{:?}", primes::mersenne_prime_basic(0, 1000));
    // println!("{:?}", primes::mersenne_prime_parallel(0, 1000));
    println!("{:?}", primes::mersenne_prime_parallel(0, 100000));
}