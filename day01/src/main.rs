use std::fs;
use std::io::BufReader;
use std::io::prelude::*;

/// Get the title of a Markdown file.
///
/// Reads the first line of a Markdown file, strips any hashes and
/// leading/trailing whitespace, and returns the title.
fn read_line<R>(mut rdr: R) -> String
    where R: BufRead,
{
    let mut first_line = String::new();

    rdr.read_line(&mut first_line).expect("Unable to read line");

    // Where do the leading hashes stop?
    let last_hash = first_line
        .char_indices()
        .skip_while(|&(_, c)| c == '#')
        .next()
        .map_or(0, |(idx, _)| idx);

    // Trim the leading hashes and any whitespace
    first_line[last_hash..].trim().into()
}

fn calculate_floor(line: String) -> i32 {
    let chars = line.chars();
    let mut floor = 0;
    for c in chars {
        if c == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }
    }
    return floor;
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

    let floor = calculate_floor(line);

    println!("The floor is '{}'", floor);
}

#[cfg(test)]
mod test {
    use super::title_string;

    #[test]
    fn top_level_heading() {
        assert_eq!(title_string(b"# This is a top-level heading".as_ref()),
                   "This is a top-level heading")
    }

    #[test]
    fn second_level_heading() {
        assert_eq!(title_string(b"## This is a second-level heading".as_ref()),
                   "This is a second-level heading");
    }

    #[test]
    fn fourth_level_heading() {
        assert_eq!(title_string(b"#### Let's jump straight to the fourth-level heading".as_ref()),
                   "Let's jump straight to the fourth-level heading");
    }
}