mod threecubes;
mod goldenratio;
mod pi;

fn main() {
    // let argument = "basic-three-cubes";
    // let argument = "fibonacci";
    let argument = "pi";
    #[allow(unused_mut)]
    let mut output; // Output variable
    #[allow(unused_mut)]
    if argument == "basic-three-cubes" {
        let mut smallestcube;
        output = threecubes::basicthreecubes(1000, 2005);
        println!("List of cubes: {:?}", output);
        smallestcube = threecubes::getsmallestcube(output);
        println!("Smallest cube: {:?}", smallestcube);
    } else if argument == "fibonacci" {
        println!("{:?}", goldenratio::goldenratio(20));
    } else if argument == "pi" {
        println!("{:?}", pi::pimultithreaded(1000000000));
        // println!("{:?}", pi::pisinglethreaded(100000000));
    } else {
        println!("Error: argument invalid");
    }
    
}