use std::{fs, collections::HashSet, hash::Hash};
use std::time::{Duration, Instant};

fn parse_assignment(assignment: &str) -> (i32, i32) { 
    let mut split_assignment = assignment.split("-");
    let lower = split_assignment.next().unwrap().parse::<i32>().unwrap();
    let upper = split_assignment.next().unwrap().parse::<i32>().unwrap();

    (lower, upper)
}

fn part1() {
    let file_path: String = String::from(".\\data\\04");
    let file_content = fs::read_to_string(file_path)
        .expect("File doesn't exist");

    let mut acc = 0;
    for line in file_content.lines() {
        let mut assignments = line.split(",");
        
        let (lower1, upper1) = parse_assignment(assignments.next().unwrap());
        let (lower2, upper2) = parse_assignment(assignments.next().unwrap()); 

        if (lower1 <= lower2 && upper1 >= upper2) ||
           (lower2 <= lower1 && upper2 >= upper1) {
            acc += 1;
        }
    }

    println!("Score: {}", acc)
}

fn part2() {
    let file_path: String = String::from(".\\data\\04");
    let file_content = fs::read_to_string(file_path)
        .expect("File doesn't exist");

    let mut acc = 0;
    for line in file_content.lines() {
        let mut assignments = line.split(",");
        
        let (lower1, upper1) = parse_assignment(assignments.next().unwrap());
        let (lower2, upper2) = parse_assignment(assignments.next().unwrap()); 

        if lower1 <= upper2 && lower2 <= upper1 {
            acc += 1;
        }
    }

    println!("Score: {}", acc)
}

fn main() {
    let now = Instant::now();
    part2();
    println!("Time elapsed is: {:?}", now.elapsed());
}