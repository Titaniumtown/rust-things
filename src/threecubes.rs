use rayon::iter::{ParallelIterator, IntoParallelIterator};

#[allow(dead_code)]
pub fn basicthreecubes(maxrange: i64, target: i64) -> Vec<[i64; 3]> {
    println!("max range: {}\ntarget: {}\ntotal possible numbers: {}", maxrange, target, i64::pow(2*maxrange, 3));

    let mut xtmp; let mut ytmp; // tmp vars for caching expontents
    let mut solnum = 0; // # of solutions found
    let mut solutions: Vec<[i64; 3]> = Vec::new(); // list of solutions found
    let mut expontents: Vec<i64> = Vec::new(); // list that holds cached exponents

    // cache exponents to be called later
    println!("precomputing exponents...");
    expontents = (-maxrange..maxrange).into_par_iter().map(|x| i64::pow(x,3)).collect();
    println!("done precomputting exponents!");


    println!("now doing the real number crunching...");
    for x in -maxrange..maxrange {
        xtmp = expontents[(x+maxrange) as usize];
        for y in -maxrange..maxrange {
            ytmp = expontents[(y+maxrange) as usize];
            for z in -maxrange..maxrange {
                if xtmp+ytmp+expontents[(z+maxrange) as usize] == target {
                    println!("found: {}, {}, {}", x, y, z);
                    solnum += 1;
                    solutions.push([x,y,z]);
                }
            }
        }
    }
    println!("total found: {}", solnum);
    return solutions;
}

#[allow(dead_code)]
pub fn threadedthreecubes(maxrange: i64, target: i64) -> Vec<[i64; 3]> {
    println!("max range: {}\ntarget: {}\ntotal possible numbers: {}", maxrange, target, i64::pow(2*maxrange, 3));

    let mut xtmp; let mut ytmp; // tmp vars for caching expontents
    let mut solnum = 0; // # of solutions found
    let mut solutions: Vec<[i64; 3]> = Vec::new(); // list of solutions found
    let mut expontents: Vec<i64> = Vec::new(); // list that holds cached exponents

    // cache exponents to be called later
    println!("precomputing exponents...");
    expontents = (-maxrange..maxrange).into_par_iter().map(|x| i64::pow(x,3)).collect();
    println!("done precomputting exponents!");
    
    let mut tmp: Vec<i64>;
    
    println!("now doing the real number crunching...");
    for x in -maxrange..maxrange {
        xtmp = expontents[(x+maxrange) as usize];
        for y in -maxrange..maxrange {
            ytmp = expontents[(y+maxrange) as usize];
            // for z in range {
            tmp = (-maxrange..maxrange).into_par_iter().filter(|&z| xtmp+ytmp+&expontents[(z+maxrange) as usize] == target).collect();
            if tmp.len() > 0 {
                for i in tmp {
                    solnum += 1;
                    solutions.push([x,y,i]);
                }
            }
        }
    }
    
    println!("total found: {}", solnum);
    return solutions;
}



#[allow(dead_code)]
pub fn getsmallestcube(inlist: Vec<[i64; 3]>) -> [i64; 3] {
    let mut output: [i64; 3] = [0, 0, 0];
    let mut smallestsum = std::i64::MAX;
    let mut procsum;
    for ele in inlist {
        procsum = (ele[0].abs())+(ele[1].abs())+(ele[2].abs());
        if smallestsum > procsum {
            output = ele;
            smallestsum = procsum;
        }
    }
    return output;
}