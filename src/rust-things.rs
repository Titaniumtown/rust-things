extern crate rayon;
extern crate primal;
extern crate indicatif;

// integers
extern crate num_bigint;
extern crate rand;
extern crate num_traits;
extern crate rug;

mod threecubes;
mod goldenratio;
mod pi;
mod primes;

#[allow(dead_code)]
enum Action {
    ThreeCubes,
    GoldenRatioCalc,
    PiCalc,
    MersennePrime
}

fn main() {
    let target_action = Action::MersennePrime;

    match target_action {
        Action::ThreeCubes => {
            let maxrange = 200;
            let target = 30;
    
            println!("Max Range: {} Target Value: {}", maxrange, target);
            let output = threecubes::basicthreecubes(maxrange, target);
            println!("List of cubes: {:?}", output);
            let smallestcube = threecubes::getsmallestcube(output);
            println!("Smallest cube: {:?}", smallestcube);
        },
        Action::GoldenRatioCalc => {
            println!("{:?}", goldenratio::goldenratio(20));
        },
        Action::PiCalc => {
            println!("{:?}", pi::pisinglethreaded(1000000000));
        },
        Action::MersennePrime => {
            println!("{:?}", primes::mersenne_prime_parallel(0, 10000, true));
        }
    }
}