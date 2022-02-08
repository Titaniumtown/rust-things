use rayon::iter::{ParallelIterator, IntoParallelRefIterator, IntoParallelIterator};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};

// For fileio
use std::io::{BufRead, Write, BufReader};
use std::fs::{File, OpenOptions};
use std::path::Path;

// Integers
use num_traits::identities::{One, Zero};
use rug::{Assign, Integer, ops::Pow};

// Cheaty method of checking if it's a merseen prime, it just checks to see if it's a known one.
#[allow(dead_code)]
fn is_mersenne_cheaty(prime: usize) -> bool {
    return [2, 3, 5, 7, 13, 17, 19, 31, 61, 89, 107, 127, 521, 607, 1279, 2203, 2281, 3217, 4253, 4423, 9689, 9941, 11213, 19937, 21701, 23209, 44497, 86243, 110503, 132049, 216091, 756839, 859433, 1257787, 1398269, 2976221, 3021377, 6972593, 13466917, 20996011, 24036583, 25964951, 30402457, 32582657, 37156667, 42643801, 43112609, 57885161, 74207281, 77232917, 82589933].contains(&prime);
}

// https://github.com/roiban1344/mersenne_primes/blob/main/packages/num_bigint/lucas_lehmer/src/main.rs
// Much faster than any other implementations (about 7x faster than bigint)
#[inline(always)]
#[allow(dead_code)]
fn lucas_lehmer_rug(p: u32) -> bool {
    //p must be an odd prime
    let m = (Integer::from(1) << p) - Integer::from(1); // (2^p)-1
    let mut s = Integer::from(4);
    let mut tmp_1 = Integer::new();
    let mut tmp_2 = Integer::new();
    for _ in 1..=p - 2 {
        s = s.pow(2) - Integer::from(2);
        while s >= m {
            tmp_1.assign(&s >> &p);
            tmp_2.assign(&s & &m);
            s.assign(&tmp_1 + &tmp_2);
            if &s == &m {
                s.assign(Integer::ZERO);
                break;
            }
        }
    }
    s == Integer::ZERO
}


#[inline]
fn basic_isprime(prime: usize) -> bool {
    if prime == 2 {return true};
    return primal::is_prime(prime as u64);
}

// Get list of already checked prime numbers from file
fn get_from_file(file_path: &str) -> Vec<i32> {
    let path = Path::new(file_path);
    let lines = lines_from_file(path);

    let resumed_primes = lines.par_iter().map(|x| {
        return str::replace(&str::replace(x, "I: ", ""), "P: ", "").parse::<i32>().unwrap();
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
pub fn mersenne_prime_parallel(start: i32, plus: i32, save_file: bool) -> Vec<i32> {
    println!("Start: {}\nPlus: {}", start, plus);

    let file_path = "./prime_file.txt";
    let path = Path::new(file_path);

    if (save_file) {
        if !path.exists() {
            File::create(file_path).expect("unable to create file");
        }
    }

    let numrange = start..(start+plus);

    let mut pre_processed: Vec<i32> = numrange.collect();

    if (save_file) {
        let lines = lines_from_file(Path::new(file_path));
        println!("# of lines: {}", lines.len());
        if lines.len() > 0 {
            let resumed_primes = get_from_file(file_path);
            println!("Loading already calculated primes...");
            pre_processed = pre_processed.into_par_iter()
                .filter(|x| !resumed_primes.contains(x))
                .collect();
        } else {
            println!("File: {} is empty, skipping", file_path);
        }
    }

    println!("Preprocessing data...");
    let pre_processed: Vec<i32> = pre_processed.into_par_iter()
        .filter(|&x| basic_isprime(x as usize)).collect();
    println!("Done preprocessing data!");

    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)
        .unwrap();


    println!("Starting to calculate primes!");
    println!("Number to test: {}", pre_processed.len());
    let pb = ProgressBar::new(pre_processed.len() as u64);
    pb.set_style(ProgressStyle::default_bar().template(
        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, {percent}%, {per_sec})",
    ));

    return pre_processed.into_par_iter().progress_with(pb)
            .filter(|&x|{
                let mut file = &file;
                let flag = lucas_lehmer_rug(x as u32);
                if (save_file) {
                    let msg: String;
                    if flag {
                        msg = format!("P: {}", x).to_string();
                        println!("{}", msg);
                    } else {
                        msg = format!("I: {}", x).to_string();
                    }
                    file.write_all(format!("{}\n", msg).as_bytes());
                }
                return flag;
                
            }).collect();
}

#[allow(dead_code)]
fn odd_num_gen(num: i32) -> Vec<i32> {
    return (1..num).into_par_iter().filter(|&x| x % 2 != 0).collect();
}

// Multi threaded implementation to find normal Primes
#[allow(dead_code)]
pub fn prime_finder_parallel(start: i32, plus: i32) -> Vec<i32> {
    let numrange = start..(start+plus);
    let output: Vec<i32> = numrange.into_par_iter().progress()
        .filter(|&x| return basic_isprime(x as usize)).collect();
    return output;
}


// Tests!!!
#[allow(dead_code)]
static TEST_RANGE: usize = 1000;

#[test]
fn test_lehmer_test_rug() {
    for i in 3..TEST_RANGE {
        assert!(is_mersenne_cheaty(i) == lucas_lehmer_rug(i as u32), true);
    }
}

#[test]
fn big_test() {
    assert!(is_mersenne_cheaty(44497) == lucas_lehmer_rug(44497), true);
}

#[test]
fn prime_parallel_test() {
    assert!(vec![3, 5, 7, 13, 17, 19, 31, 61, 89, 107, 127, 521, 607, 1279, 2203, 2281, 3217, 4253, 4423, 9689, 9941] == mersenne_prime_parallel(0, 10000, false), true);
}