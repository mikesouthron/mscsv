use std::env;

extern crate mscsv;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let sep;
    if args.len() < 3 {
        sep = ',';
    } else {
        sep = args[2].as_bytes()[0] as char;
    }

    mscsv::parse_csv(path, sep);
}
