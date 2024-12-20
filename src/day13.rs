use regex::Regex;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use ndarray::prelude::*;
use ndarray_linalg::Solve;
use ndarray_linalg::*;

use num_bigint::BigInt;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// solve ax + by = e
//       cx + dy = f
fn solve_linear(a: u64, b: u64, c: u64, d: u64, e: u64, f: u64) -> Option<(BigInt, BigInt)> {
    let a = BigInt::from(a);
    let b = BigInt::from(b);
    let c = BigInt::from(c);
    let d = BigInt::from(d);
    let e = BigInt::from(e);
    let f = BigInt::from(f);
    let det = &a * &d - &b * &c;
    let x;
    if (&d * &e - &b * &f) % (&det) == BigInt::from(0 as u64) {
        x = (&d * &e - &b * &f) / &det;
    } else {
        return None
    }

    let y;
    if (&a * &f - &c * &e) % (&det) == BigInt::from(0 as u64) {
        y = (&a * &f - &c * &e) / &det;
    } else {
        return None
    }

    Some((x, y))
}

pub fn read_file() {
    let buttonARE = Regex::new(r"Button A: X\+([0-9]+), Y\+([0-9]+)").unwrap();
    let buttonBRE = Regex::new(r"Button B: X\+([0-9]+), Y\+([0-9]+)").unwrap();
    let prizeRE = Regex::new(r"Prize: X=([0-9]+), Y=([0-9]+)").unwrap();
    let lines: Vec<_> = read_lines("data/day13/data.txt").unwrap().map(|r|r.unwrap()).collect();
    let mut total = BigInt::ZERO;
    let mut count = 0;
    for i in (0..lines.len()).step_by(4) {
        let buttonA = lines.get(i).unwrap();
        let buttonB = lines.get(i + 1).unwrap();
        let prize = lines.get(i + 2).unwrap();
        let capsA = buttonARE.captures(&buttonA).unwrap();
        let AX: u64 = str::parse(capsA.get(1).unwrap().as_str()).unwrap();
        let AY: u64 = str::parse(capsA.get(2).unwrap().as_str()).unwrap();
        
        let capsB = buttonBRE.captures(&buttonB).unwrap();
        let BX: u64 = str::parse(capsB.get(1).unwrap().as_str()).unwrap();
        let BY: u64 = str::parse(capsB.get(2).unwrap().as_str()).unwrap();

        let capsP = prizeRE.captures(prize).unwrap();
        let PX: u64 = str::parse(capsP.get(1).unwrap().as_str()).unwrap();
        let PY: u64 = str::parse(capsP.get(2).unwrap().as_str()).unwrap();

        let sol = solve_linear(AX, BX, AY, BY, PX + 10000000000000, PY + 10000000000000);
        if let Some((x, y)) = sol {
            total += 3 * x + y;
        }
    }
    dbg!(total);

}

