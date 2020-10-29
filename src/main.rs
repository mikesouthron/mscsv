use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let sep;
    if args.len() < 3 {
        sep = ',';
    } else {
        sep = args[2].as_bytes()[0] as char;
    }

    parse_csv(path, sep);
}

fn parse_csv(path: &String, sep: char) {
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

fn parse_header(row: String, sep: char, headers: &mut HashMap<String, u32>) {}

fn parse_row(row: String, sep: char) -> Vec<String> {
    let mut vec = Vec::new();

    vec
}
