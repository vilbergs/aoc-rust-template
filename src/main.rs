use std::fs;
use std::time::Instant;

mod utils;

fn main() {
    let filename = 
        //"./sample.txt"
        "./input.txt";

    let input = fs::read_to_string(filename).expect("Could not read input");

    part1(input.clone());
    part2(input);
}

fn part1(input: String) {
    println!("=======================");
    println!("Running Part 1");
    println!("=======================");

    let now = Instant::now();
    
    
    // Implement
    
    let elapsed = now.elapsed();
    println!("-----------------------");
    println!("Part 1 ran in: {:.2?}", elapsed);
    println!("-----------------------");
}

fn part2(input: String) {
    println!("=======================");
    println!("Running Part 2");
    println!("=======================");
    
    let now = Instant::now();
    
    
    // Implement
    
    let elapsed = now.elapsed();
    println!("-----------------------");
    println!("Part 2 ran in: {:.2?}", elapsed);
    println!("-----------------------");
}
