use arrayfire as af;
// use arrayfire::*;
use std::time::Instant;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

// Based off of: https://github.com/arrayfire/arrayfire-rust/blob/master/examples/pi.rs
pub fn pi_cl() {
    // backend_man();
    af::set_backend(af::Backend::CPU);
    af::set_device(0);
    af::info();

    let samples = 20_000_000;
    let dims = af::Dim4::new(&[samples, 1, 1, 1]);

    // /*
    let x = &af::randu::<f64>(dims);
    let y = &af::randu::<f64>(dims);
    // */

    let start = Instant::now();

    af::mem_info!("Before");

    let xsqrd = &af::mul(x, x, false);
    let ysqrd = &af::mul(y, y, false);
    let xplusy = &af::add(xsqrd, ysqrd, false);
    let root = &af::sqrt(xplusy);
    let cnst = &af::constant(1, dims);
    let (real, imag) = af::sum_all(&af::le(root, cnst, false));
    let pi_val = (real as f64) * 4.0 / (samples as f64);
   

    println!("Pi Value: {:?}", pi_val);
    println!("Took: {:?}", start.elapsed());

    af::mem_info!("After");
}

pub fn prime_cl() {
    backend_man();
    af::set_device(0);
    af::info();

    let samples = 20_000_000;
    let dims = af::Dim4::new(&[samples, 1, 1, 1]);
    let nums = &af::randu::<i64>(dims);

}


fn backend_man() {
    let available = af::get_available_backends();

    if available.contains(&af::Backend::CUDA) {
        println!("Evaluating CUDA Backend...");
        af::set_backend(af::Backend::CUDA);
        println!("There are {} CUDA compute devices", af::device_count());
        return;
    }

    if available.contains(&af::Backend::OPENCL) {
        println!("Evaluating OpenCL Backend...");
        af::set_backend(af::Backend::OPENCL);
        println!("There are {} OpenCL compute devices", af::device_count());
        return;
    }

    if available.contains(&af::Backend::CPU) {
        println!("Evaluating CPU Backend...");
        af::set_backend(af::Backend::CPU);
        println!("There are {} CPU compute devices", af::device_count());
        return;
    }
}