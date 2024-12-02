use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

fn print_distance(list1: &Vec<u32>, list2: &Vec<u32>){
    let mut res: Vec<u32> = Vec::new();

    assert!(list1.len() == list2.len());
    for i in 0..list1.len(){
        let val1 = list1.get(i).unwrap();
        let val2 = list2.get(i).unwrap();
        if val1>=val2 {
            res.push(val1-val2);
        }
        else {
            res.push(val2-val1);
        }
    }

    println!("{}",res.iter().sum::<u32>());
}

fn print_similarity(list1: &Vec<u32>, list2: &Vec<u32>){
    let mut map: HashMap<u32, u32> = HashMap::new();
    for val in list2.iter() {
        map.insert(*val, *(map.get(val).unwrap_or(&0))+1);
    }
    let res: u32 = list1.iter().map(|x| x*(map.get(x).unwrap_or(&0))).sum();
    println!("{:?}", res);
}

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let br = BufReader::new(f);

    let mut list1: Vec<u32> = Vec::new();
    let mut list2: Vec<u32> = Vec::new();

    for line in br.lines() {
        match line {
            Ok(s) => {
                let vals = s.split_once(' ').unwrap();
                list1.push(vals.0.trim().parse().unwrap());
                list2.push(vals.1.trim().parse().unwrap());
            },
            Err(_) => panic!("Error while reading"),
        }
    }

    list1.sort();
    list2.sort();

    print_distance(&list1, &list2);
    print_similarity(&list1,&list2);

    Ok(())
}