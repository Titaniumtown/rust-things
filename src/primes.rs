use rayon::iter::{ParallelIterator, IntoParallelRefIterator, IntoParallelIterator};
use num_integer::Integer;
use ramp::Int;
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};

// for file stuff
use std::io::{self, BufRead, Write, Read, BufReader};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::fs::OpenOptions;

fn is_mersenne(prime: usize) -> bool {
    /*
    if prime == 2 {return true};

    if !primal::is_prime(prime as u64) || prime.is_even() {
        return false;
    */

    let mersenne: Int = Int::from(Int::from(2).pow(prime) - 1);
    let mut s: Int = Int::from(4);

    for _i in 2..prime {
        s = (&s * &s - 2) % &mersenne;
    }

    return s == 0;
}

fn basic_mersenne_check(prime: usize) -> bool { // used for first pass
    if prime == 2 {return true};

    if !primal::is_prime(prime as u64) || prime.is_even() {
        return false;
    }

    return true
}


fn get_resumed_primes(start: i32, plus: i32, file_path: &str) -> Vec<i32> {
    let mut path = Path::new(file_path);
    let mut file = File::open(&path).expect("Unable to open");
    let buffer = BufReader::new(file);
    let mut lines = lines_from_file(path);
    lines.remove(0);
    let mut tmp: String = String::new();
    let mut ele: String = String::new();

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
    /*
    return buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
    */
}

fn init_prime_file(start: i32, plus: i32, file_path: &str) {
    let init_line = format!("start({}) plus({})", start, plus).to_string();
    let mut path = Path::new(file_path);
    let mut file = File::open(&path).expect("Unable to open");


    let buffer = BufReader::new(file);
    let lines = lines_from_file(path);
    let init_line_write = format!("{}\n", init_line).to_string();

    if !path.exists() || lines.len() == 0 {
        file = File::create(file_path).expect("unable to create file");
        file.write_all(init_line_write.as_bytes()).expect("unable to write");
        return;
    }

    let firstline = lines.first().unwrap();
    println!("{}", firstline);
    if firstline != &init_line {
        println!("Warning, init lines don't match, overwriting...");
        file = File::create(file_path).expect("unable to create file");
        file.write_all(init_line_write.as_bytes()).expect("unable to write");
    }
}

// Multi threaded implementation to find Mersenne Primes
#[allow(dead_code)]
pub fn mersenne_prime_parallel(start: i32, plus: i32, options: Vec<i16>) -> Vec<i32> {
    let file_path = "./prime_file.txt";
    if options[0] == 1 {
        init_prime_file(start, plus, file_path);
    }
    let mut numrange = (start..(start+plus));
    let pb = ProgressBar::new(numrange.len() as u64);
    pb.set_style(ProgressStyle::default_bar().template(
        "{spinner:.green} [{elapsed_precise}] [{bar:20.cyan/blue}] ({pos}/{len}, ETA {eta})",
    ));

    println!("Preprocessing data...");
    let mut pre_processed: Vec<i32> = numrange.into_par_iter().progress()
        .filter(|&x| basic_mersenne_check(x as usize)).collect();
    println!("Done preprocessing data!");

    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)
        .unwrap();

    let mut output: Vec<i32> = Vec::new();
    if options[0] == 1 {
        let resumed_primes = get_resumed_primes(start, plus, file_path);
        println!("Loading already calculated primes...");
        pre_processed = pre_processed.into_par_iter().progress()
            .filter(|x| !resumed_primes.contains(x))
            .collect();
        output = pre_processed.into_par_iter().progress_with(pb)
            .filter(|&x|{
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
                
            }).collect();
            
            
    } else {
        output = pre_processed.into_par_iter().progress_with(pb)
            .filter(|&x| {
                if is_mersenne(x as usize) {
                    println!("Prime: {}", x);
                    return true;
                }
                return false;
            }).collect();
    }
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