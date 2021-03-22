#[allow(dead_code)]
pub fn goldenratio(n: i64) -> f64 {
    let mut a = 1;
    let mut b = 1;
    let mut tmp: i64;
    for _i in 0..n {
        tmp = b;
        b = a+b;
        a = tmp;
    }
    return (b as f64)/(a as f64);
}