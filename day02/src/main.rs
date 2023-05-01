use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "./input.txt";
    let mut total = 0;
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        let measurements = line.split("x");

        // Parse Split to Vector of ints
        let dimensions: Vec<i32> = measurements.map(|x| x.parse::<i32>().unwrap()).collect();
        total += parse_box(dimensions)
    }
    println!("Total: {}", total);
}

fn parse_box(dimensions: Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    let mut min = dimensions[0];
    for dim in dimensions {
        if dim < min {
            min = dim;
        }
        sum += 2 * dim;
    }
    sum += min;
    println!("{}", sum);
    sum
}
