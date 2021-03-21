pub fn goldenratio(n: i64) -> f64 {
    let mut a = 1;
    let mut b = 1;
    let mut tmp: i64 = 0;
    for i in 0..n {
        tmp = b;
        b = a+b;
        a = tmp;
    }
    return (b as f64)/(a as f64);
}