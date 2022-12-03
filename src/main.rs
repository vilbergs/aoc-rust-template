use std::fs;
use std::time::Instant;

mod utils;

fn main() {
    let filename = 
        //"./sample.txt"
        "./input.txt";

    let input = fs::read_to_string(filename).expect("Could not read input");
    let input_part_2 = input.clone(); // Cloning input here so it doesn't affect benchmarks

    println!("=======================");
    println!("Running Part 1");
    println!("=======================");

    let now = Instant::now();
    
    part1(input);

    let elapsed = now.elapsed();
    println!("-----------------------");
    println!("Part 1 ran in: {:.2?}", elapsed);
    println!("-----------------------");

    println!("=======================");
    println!("Running Part 2");
    println!("=======================");
    
    let now = Instant::now();

    part2(input_part_2);

    let elapsed = now.elapsed();
    println!("-----------------------");
    println!("Part 2 ran in: {:.2?}", elapsed);
    println!("-----------------------");
}

fn part1(input: String) {
    todo!("Implement part 1");
}

fn part2(input: String) {
    todo!("Implement part 1");
}
