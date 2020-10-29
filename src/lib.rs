use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

pub fn parse_csv(path: &String, sep: char) {
    let file = File::open(path).expect("Unable to open file");
    let lines = io::BufReader::new(file).lines();
    let mut headers: HashMap<String, u32> = HashMap::new();
    let mut done_header = false;
    for line in lines {
        if let Ok(row) = line {
            println!("{}", row);
            if !done_header {
                parse_header(row, sep, &mut headers);
                done_header = true;
            } else {
                let row_vec = parse_row(row, sep);
                println!("{:?}", row_vec);
            }
        }
    }
}

fn parse_header(row: String, sep: char, headers: &mut HashMap<String, u32>) {
    println!("{}, {}, {:?}", row, sep, headers);
}

fn parse_row(row: String, sep: char) -> Vec<String> {
    let mut vec = Vec::new();

    println!("{}, {}, {:?}", row, sep, vec);

    vec
}
