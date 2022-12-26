#[allow(dead_code)]

use std::fs::File;
use std::io::{BufReader, BufRead};

struct Point {
    x: i32,
    y: i32,
}
struct Sheet {
    point1: Point,
    point2: Point,
}

fn load_from_file(file_path: &str) {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let _numbers: Vec<i64> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect();
}

fn main() {
    load_from_file("BAI5.inp");
}
