use rayon::iter::{ParallelIterator, IntoParallelRefIterator, IntoParallelIterator};
// use rayon::prelude::*;

use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};

// for file stuff
use std::io::{BufRead, Write, BufReader};
use std::fs::File;
use std::path::Path;
use std::fs::OpenOptions;

// integers
use ramp::Int;
use num_bigint::{BigUint, ToBigUint};

fn is_mersenne(prime: usize) -> bool {
    // num-bigint implementation:
    /*
    let zero: BigUint = 0.to_biguint().unwrap();
    let one: BigUint = 1.to_biguint().unwrap();
    let two: BigUint = 2.to_biguint().unwrap();

    let mersenne: BigUint = two.pow(prime as u32) - one;
    let mut s: BigUint = 4.to_biguint().unwrap();


    for _i in 2..prime {
        s = (&s.pow(2) - &two) % &mersenne;
    }

    return s == zero;
    */

    // ramp implementation:
    let mersenne: Int = Int::from(2).pow(prime) - 1;
    let mut s: Int = Int::from(4);

    for _i in 2..prime {
        s = (&s.pow(2) - 2) % &mersenne;
    }

    return s == 0;
}

fn basic_mersenne_check(prime: usize) -> bool { // used for first pass
    if prime == 2 {return true};

    if !primal::is_prime(prime as u64) || prime % 2 == 0 {
        return false;
    }

    return true
}


fn get_resumed_primes(file_path: &str) -> Vec<i32> {
    let path = Path::new(file_path);
    let lines = lines_from_file(path);

    let resumed_primes = lines.par_iter().map(|x| {
        return str::replace(&str::replace(x, "Invalid: ", ""), "Prime: ", "").parse::<i32>().unwrap();
    }).collect();
    return resumed_primes;
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    let mut output: Vec<String> = Vec::new();
    for line in buf.lines() {
        output.push(line.expect("Could not parse line"));
    }
    return output;
}

// Multi threaded implementation to find Mersenne Primes
#[allow(dead_code)]
#[inline(always)]
pub fn mersenne_prime_parallel(start: i32, plus: i32, options: Vec<i16>) -> Vec<i32> {
    let debug: bool = true;
    println!("Start: {}\nPlus: {}", start, plus);

    let file_path = "./prime_file.txt";
    let path = Path::new(file_path);
    if !path.exists() {
        File::create(file_path).expect("unable to create file");
    }

    let numrange = start..(start+plus);


    println!("Preprocessing data...");
    let mut pre_processed: Vec<i32> = numrange.into_par_iter()
        .filter(|&x| basic_mersenne_check(x as usize)).collect();
    println!("Done preprocessing data!");

    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)
        .unwrap();

    let mut output: Vec<i32> = Vec::new();
    let lines = lines_from_file(Path::new(file_path));
    println!("# of lines: {}", lines.len());
    if lines.len() > 0 {
        let resumed_primes = get_resumed_primes(file_path);
        println!("Loading already calculated primes...");
        pre_processed = pre_processed.into_par_iter()
            .filter(|x| !resumed_primes.contains(x))
            .collect();
    } else {
        println!("File: {} is empty, skipping", file_path);
    }
    println!("Starting to calculate primes!");
    println!("Number to test: {}", pre_processed.len());
    let pb = ProgressBar::new(pre_processed.len() as u64);
    pb.set_style(ProgressStyle::default_bar().template(
        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, {percent}%, {per_sec})",
    ));
    /*
    let thread_num = 24;
    let chunk_size = pre_processed.len()/thread_num;

    output = pre_processed.par_chunks(chunk_size).progress_with(pb)
        .filter(|chunk|
            chunk.simd_iter().simd_filter(|&x|{
                let mut file = &file;
                if is_mersenne(x as usize) {
                    let msg = format!("Prime: {}", x).to_string();
                    println!("{}", msg);
                    file.write_all(format!("{}\n", msg).as_bytes());
                    return true;
                } 
                let msg = format!("Invalid: {}", x).to_string();
                file.write_all(format!("{}\n", msg).as_bytes());
                return false;
            }).scalar_collect();
        }).collect();
    */

    // /*
    output = pre_processed.into_par_iter().progress_with(pb)
        .filter(|&x|{
            let mut file = &file;
            let flag = is_mersenne(x as usize);
            let mut msg: String = String::new();
            if flag {
                msg = format!("Prime: {}", x).to_string();
                println!("{}", msg);
            } else {
                msg = format!("Invalid: {}", x).to_string();
            }
            file.write_all(format!("{}\n", msg).as_bytes());
            return flag;
            
        }).collect();
    // */
    return output;
}

// Multi threaded implementation to find normal Primes
#[allow(dead_code)]
pub fn prime_finder_parallel(start: i32, plus: i32) -> Vec<i32> {
    let numrange = start..(start+plus);
    let output: Vec<i32> = numrange.into_par_iter().progress()
        .filter(|&x| return primal::is_prime(x as u64)).collect();
    return output;
}