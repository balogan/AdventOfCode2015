use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::Chars;

/// Get the title of a Markdown file.
///
/// Reads the first line of a Markdown file, strips any hashes and
/// leading/trailing whitespace, and returns the title.
fn read_line<R>(mut rdr: R) -> String
where
    R: BufRead,
{
    let mut first_line = String::new();

    rdr.read_line(&mut first_line).expect("Unable to read line");

    first_line
}

fn calculate_floor(chars: Chars) -> i32 {
    let mut floor = 0;
    for c in chars {
        if c == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }
    }
    floor
}

fn get_basement_count(chars: Chars) -> i32 {
    let mut floor = 0;
    let mut basement_count = 0;
    for c in chars {
        if c == '(' {
            floor += 1;
            basement_count += 1;
        } else {
            floor -= 1;
            basement_count += 1;
        }
        if floor == -1 {
            return basement_count;
        }
    }
    basement_count
}

/// Read the first line of the file into `title`.
fn main() {
    let path = "./input.txt";

    let file = match fs::File::open(path) {
        Ok(file) => file,
        Err(_) => panic!("Unable to open file {}", path),
    };
    let buffer = BufReader::new(file);

    let line = read_line(buffer);
    // TODO: deduplicate this
    let chars = line.chars();
    let chars2 = line.chars();

    // TODO possibly collapse to single function
    let floor = calculate_floor(chars);
    let basement_count = get_basement_count(chars2);

    println!("The floor is {}", floor);
    println!("The basement count is {}", basement_count);
}
