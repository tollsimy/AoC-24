use std::fs::File;
use std::io::{BufRead, BufReader};

fn check_val(vals: &Vec<i32>) -> bool {
    let mut res = true;
    let mut prev: i32 = 0;
    let mut cur: i32;

    for i in 0..vals.len() - 1 {
        cur = (vals[i] - vals[i + 1]).try_into().unwrap();

        if !(cur.abs() >= 1 && cur.abs() <= 3) {
            res = false;
        }
        if prev * cur < 0 {
            res = false;
        }
        prev = cur;
    }
    res
}

fn check_val2(vals: &Vec<i32>) -> bool {
    let mut res = if check_val(vals) {1} else {0};

    for i in 0..vals.len() {
        let mut vals_clone = vals.clone();
        vals_clone.remove(i);
        res = if check_val(&vals_clone) {res+1} else {res};
    }
    return res > 0;
}

fn main() -> std::io::Result<()> {
    let file: File = File::open("input.txt")?;
    let rb = BufReader::new(file);

    let mut res: u32 = 0;
    let mut res2: u32 = 0;
    for line in rb.lines() {
        match line {
            Ok(line) => {
                let vals = line
                    .split_ascii_whitespace()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect();
                res = if check_val(&vals) {res+1} else {res};
                res2 = if check_val2(&vals) {res2+1} else {res2};
            }
            Err(_) => panic!("Err reading file"),
        }
    }

    println!("{}", res);
    println!("{}", res2);

    Ok(())
}
