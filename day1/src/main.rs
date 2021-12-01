use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut result = 0;
    let mut last_input: [i32; 3] = [0;3];
    let mut last_sum: i32 = 0;
    // File must exist in current path before this produces output
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {

            if let Ok(string_input) = line {
                let int_input: i32 = string_input.parse().unwrap();

                if last_input[0] != 0 &&
                    (last_input[1] + last_input[2] + int_input) >
                    (last_input[0] + last_input[1] + last_input[2])
                {
                    result += 1;
                }

                last_input[0] = last_input[1];
                last_input[1] = last_input[2];
                last_input[2] = int_input;
            }
        }
        println!{"Result is {}", result};
    }
    else {
        println!{"File {} does not exist", filename};
        return;
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
