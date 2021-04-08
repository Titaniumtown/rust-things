use rug::Float;
use indicatif::ProgressBar;

pub fn test(start: i32, iterations: i64, precision: u32) -> Float {
    println!("input: {:?}\npercision: {:?}\n", start, precision);
    let bar = ProgressBar::new(1000);
    let mut tmp: Float = Float::with_val(precision, start);

    for _ in 0..iterations {
        tmp = tmp.cos();
        bar.inc(1);
    }
    return tmp;
}
