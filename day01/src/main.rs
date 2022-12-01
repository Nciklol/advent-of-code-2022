use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn read_input() -> io::Result<Vec<String>> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);
    let mut input: Vec<String> = vec![];

    for line in reader.lines() {
        input.push(line.unwrap());
    } 

    Ok(input)
}

fn main() -> io::Result<()> {
    
    let input = read_input().unwrap();

    let mut top1 = 0;
    let mut top2 = 0;
    let mut top3 = 0;

    let mut temp = 0;



    for line in input {
        if line == "" {
            check_val(&mut temp, &mut top1, &mut top2, &mut top3);
            temp = 0;
        } else {
            temp += line.parse::<i32>().unwrap();
        }   
    }
    // For the last input case
    check_val(&mut temp, &mut top1, &mut top2, &mut top3);

    println!("{} {} {}", top1, top2, top3);
    println!("{} ", top1 + top2 + top3);
    Ok(())
}

fn check_val(temp: &mut i32, top1: &mut i32, top2: &mut i32, top3: &mut i32) {
    if *temp > *top1 && *top1 != 1 {
        check_val(top1, &mut 1, top2, top3);
        *top1 = *temp;
    } else if *temp > *top2 && *top2 != 1 {
        check_val(top2, top1, &mut 1, top3);
        *top2 = *temp;
    } else if *temp > *top3 && *top3 != 1 {
        check_val(top3, top1, top2, &mut 0);
        *top3 = *temp;
    }
}