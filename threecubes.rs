fn threecubes(maxrange: i64, target: i64) {
    let mut xtmp; let mut ytmp; let mut ztmp; // tmp vars for caching expontents
    let mut solnum = 0; // # of solutions found
    println!("max range: {}\ntarget: {}\ntotal possible numbers: {}", maxrange, target, i64::pow(2*maxrange, 3));
    
    let mut expontents = Vec::new(); // list that holds cached exponents

    // cache exponents to be called later
    println!("precomputing exponents...");
    for x in -maxrange..maxrange {
        expontents.push(i64::pow(x,3));
    }
    println!("done precomputting exponents!");


    println!("now doing the real number crunching...");
    for x in -maxrange..maxrange {
        xtmp = expontents[(x+maxrange) as usize];
        for y in -maxrange..maxrange {
            ytmp = expontents[(y+maxrange) as usize];
            for z in -maxrange..maxrange {
                ztmp = expontents[(z+maxrange) as usize];
                if xtmp+ytmp+ztmp == target {
                    println!("found: {}, {}, {}", x, y, z);
                    solnum += 1;
                }
            }
        }
    }
    println!("total found: {}", solnum);
}

fn main() {
    threecubes(1000, 999);
}