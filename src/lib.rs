use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Lines};
use std::result::Result;

pub struct CSV {
    headers: HashMap<String, u32>,
    sep: char,
    lines: Lines<std::io::BufReader<std::fs::File>>,
    current: Option<Vec<String>>,
}

impl CSV {
    pub fn get(&self, col: &str) -> Option<String> {
        match &self.current {
            Some(current) => Some(current[self.headers[col] as usize].to_string()),
            None => None,
        }
    }

    pub fn get_with(&self, current: Vec<String>, col: &str) -> String {
        current[self.headers[col] as usize].to_string()
    }
}

impl Iterator for CSV {
    type Item = Vec<String>;
    fn next(&mut self) -> Option<Vec<String>> {
        match self.lines.next() {
            Some(Ok(row)) => {
                //FIXME: Having to parse row twice because can't figure out how to re-use the returned vector
                //Could not store current, but that means being unable to use get function as is
                // self.current = Some(parse_row(&row, self.sep));
                Some(parse_row(&row, self.sep))
            }
            Some(Err(_)) => None,
            None => None,
        }
    }
}

pub fn parse_csv(path: &String, sep: char) -> Result<CSV, io::Error> {
    let file = File::open(path).expect("Unable to open file");
    let mut lines = io::BufReader::new(file).lines();
    let mut headers: HashMap<String, u32> = HashMap::new();
    if let Some(Ok(row)) = lines.next() {
        parse_header(row, ',', &mut headers);
    }

    Ok(CSV {
        headers: headers,
        sep: sep,
        lines: lines,
        current: None,
    })
}

fn parse_header(row: String, sep: char, headers: &mut HashMap<String, u32>) {
    let mut current_field = String::new();
    let mut in_quotes = false;
    let mut count: u32 = 0;
    for c in row.chars() {
        if c == '"' {
            in_quotes = !in_quotes;
        } else if c == sep && !in_quotes {
            headers.insert(current_field, count);
            current_field = String::new();
            count = count + 1;
        } else {
            current_field.push(c);
        }
    }
    headers.insert(current_field, count);
}

fn parse_row(row: &String, sep: char) -> Vec<String> {
    let mut vec = Vec::new();
    let mut in_quotes = false;
    let mut current_field = String::new();
    for c in row.chars() {
        if c == '"' {
            in_quotes = !in_quotes;
        } else if c == sep && !in_quotes {
            vec.push(current_field);
            current_field = String::new();
        } else {
            current_field.push(c);
        }
    }
    vec.push(current_field);
    vec
}

#[test]
fn test_parse_header() {
    let file = File::open("test.csv").expect("Unable to open file");
    let mut lines = io::BufReader::new(file).lines();
    let mut headers: HashMap<String, u32> = HashMap::new();
    if let Some(Ok(row)) = lines.next() {
        parse_header(row, ',', &mut headers);
    }
    assert_eq!(
        headers,
        [
            (String::from("col1"), 0),
            (String::from("col2"), 1),
            (String::from("col3"), 2),
            (String::from("col4"), 3),
        ]
        .iter()
        .cloned()
        .collect()
    );
}

#[test]
fn test_parse_row() {
    let file = File::open("test.csv").expect("Unable to open file");
    let mut lines = io::BufReader::new(file).lines();
    lines.next();
    if let Some(Ok(row)) = lines.next() {
        let vec = parse_row(&row, ',');
        assert_eq!(vec, ["val11", "val12", "val13,long1", "val14"]);
    }
    if let Some(Ok(row)) = lines.next() {
        let vec = parse_row(&row, ',');
        assert_eq!(vec, ["val21", "val22", "val23", "val24"]);
    }
}

#[test]
fn test_parse_csv() {
    let csv_result = parse_csv(&String::from("test.csv"), ',');
    if let Ok(mut csv) = csv_result {
        assert_eq!(
            csv.headers,
            [
                (String::from("col1"), 0),
                (String::from("col2"), 1),
                (String::from("col3"), 2),
                (String::from("col4"), 3),
            ]
            .iter()
            .cloned()
            .collect()
        );

        if let Some(row) = csv.next() {
            assert_eq!(row, ["val11", "val12", "val13,long1", "val14"]);
            assert_eq!(csv.get_with(row, "col2"), "val12");
            // assert_eq!(
            //     csv.current,
            //     Some(vec![
            //         String::from("val11"),
            //         String::from("val12"),
            //         String::from("val13,long1"),
            //         String::from("val14")
            //     ])
            // );
        }
        if let Some(row) = csv.next() {
            assert_eq!(row, ["val21", "val22", "val23", "val24"]);
            assert_eq!(csv.get_with(row, "col3"), "val23");
            // assert_eq!(
            //     csv.current,
            //     Some(vec![
            //         String::from("val21"),
            //         String::from("val22"),
            //         String::from("val23"),
            //         String::from("val24")
            //     ])
            // );
        }
    }
}
