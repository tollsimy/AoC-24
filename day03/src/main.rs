use std::fs::File;
use std::io::{BufReader, Read};

static MAXLENGHT: usize = 12;
fn part1(text: &mut String) -> usize {
    let mut res = 0;

    let mut index = 0;
    while index < text.len() {
        match text.find("mul(") {
            Some(i) => {
                index = i;
                let substr = text.get(i..std::cmp::min(i+MAXLENGHT, text.len())).unwrap();
                let substr = substr.replace("mul(", "");
                if substr.contains(')') {
                    let substr = substr.split(')').nth(0).unwrap_or("");
                    if let Some((op1, op2)) = substr.split_once(',') {
                        let op1: usize = if op1.parse().unwrap_or(0) <= 999 {op1.parse().unwrap_or(0)} else {0};
                        let op2: usize = if op2.parse().unwrap_or(0) <= 999 {op2.parse().unwrap_or(0)} else {0};
                        res += op1*op2;
                    }
                }
            },
            None => {
                break;
            },
        }

        // Remove searched pattern "mul("
        text.replace_range(index..index+3, "");
    }
    return res;
}

fn part2(text: &mut String) -> usize {
    let mut res = 0;

    // Split in substring after "do()"
    let todos = text.split("do()");
    // Remove part after "don't()" if any
    let todos: Vec<&str> = todos.map(|todo| todo.split("don't()").nth(0).unwrap_or("")).collect();
    // now that we removed substrings between "do()" and "don't()
    // calculate for each substring
    for todo in todos {
        res += part1(&mut todo.to_owned());
    }
    res
}

fn main() -> std::io::Result<()> {
    let file: File = File::open("input.txt")?;
    let mut rb = BufReader::new(file);

    let mut text: String = String::default();
    let _ = rb.read_to_string(&mut text);

    let mut text_clone = text.clone();
    println!("{}", part1(&mut text_clone));

    let mut text_clone = text.clone();
    println!("{}", part2(&mut text_clone));

    Ok(())
}