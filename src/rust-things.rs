mod threecubes;

fn main() {
    let argument = "--basic-three-cubes";
    #[allow(unused_mut)]
    let mut output; // Output variable
    #[allow(unused_mut)]
    let mut smallestcube;
    if argument == "--basic-three-cubes" {
        output = threecubes::basicthreecubes(1000, 69420);
        println!("List of cubes: {:?}", output);
        smallestcube = threecubes::getsmallestcube(output);
        println!("Smallest cube: {:?}", smallestcube);
        
    } else {
        println!("Error: argument invalid");
    }
    
}