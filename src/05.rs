use regex::Regex;
use std::collections::VecDeque;
use std::iter::Peekable;
use std::path::Iter;
use std::str::Lines;
use std::{fs, collections::HashSet, hash::Hash};
use std::time::{Duration, Instant};

fn part1() {
    let file_path: String = String::from(".\\data\\05");
    let file_content = fs::read_to_string(file_path)
        .expect("File doesn't exist");
    let mut lines = file_content.lines().peekable();

    let nb_crates = (lines.peek().unwrap().len() + 1) / 4;
    let mut crates = Vec::<VecDeque<String>>::with_capacity(nb_crates);
    for i in 0..nb_crates {
        crates.push(VecDeque::new());
    }
    
    let mut i;
    let mut line_chars;
    loop {
        i = 0;
        let line = lines.next().unwrap().to_string();
        line_chars = line.chars().peekable();
        if line.starts_with(" 1 ") {
            lines.next();
            break;
        }
        
        loop {
            let first_char = line_chars.next().unwrap();
            if first_char == '[' {
                crates[i].push_back(line_chars.next().unwrap().to_string());
            } else {
                line_chars.next().unwrap();
            }
            line_chars.next().unwrap();

            if line_chars.peek().is_none() {
                break;
            } else {
                line_chars.next().unwrap();
            }
            i += 1;
        }
    }

    let line_parse_regex = Regex::new("move (?P<nb>\\d+) from (?P<from>\\d+) to (?P<to>\\d+)").unwrap();
    loop { 
        let line_opt = lines.next();
        if line_opt.is_none() {
            break;
        }
        let line = line_opt.unwrap();

        let caps = line_parse_regex.captures(line).unwrap();

        let nb = str::parse::<i32>(&caps["nb"]).unwrap();
        let from = str::parse::<i32>(&caps["from"]).unwrap();
        let to = str::parse::<i32>(&caps["to"]).unwrap();

        for _i in 0..nb {
            if crates[(from - 1) as usize].is_empty() {
                break;
            }

            let from_val;
            {
                let from_crate = &mut (crates[(from - 1) as usize]);
                from_val = from_crate.pop_front().unwrap();
            }

            {
                let to_crate = &mut (crates[(to - 1) as usize]);
                to_crate.push_front(from_val.to_string());
            }
        }
    }

    let mut result = "".to_owned();
    for crate_vec in crates {
        result.push_str(crate_vec.front().unwrap());
    }

    println!("Result: {}", result)
}

fn part2() {
    let file_path: String = String::from(".\\data\\05");
    let file_content = fs::read_to_string(file_path)
        .expect("File doesn't exist");
    let mut lines = file_content.lines().peekable();

    let nb_crates = (lines.peek().unwrap().len() + 1) / 4;
    let mut crates = Vec::<VecDeque<String>>::with_capacity(nb_crates);
    for i in 0..nb_crates {
        crates.push(VecDeque::new());
    }
    
    let mut i;
    let mut line_chars;
    loop {
        i = 0;
        let line = lines.next().unwrap().to_string();
        line_chars = line.chars().peekable();
        if line.starts_with(" 1 ") {
            lines.next();
            break;
        }
        
        loop {
            let first_char = line_chars.next().unwrap();
            if first_char == '[' {
                crates[i].push_back(line_chars.next().unwrap().to_string());
            } else {
                line_chars.next().unwrap();
            }
            line_chars.next().unwrap();

            if line_chars.peek().is_none() {
                break;
            } else {
                line_chars.next().unwrap();
            }
            i += 1;
        }
    }

    let line_parse_regex = Regex::new("move (?P<nb>\\d+) from (?P<from>\\d+) to (?P<to>\\d+)").unwrap();
    loop { 
        let line_opt = lines.next();
        if line_opt.is_none() {
            break;
        }
        let line = line_opt.unwrap();

        let caps = line_parse_regex.captures(line).unwrap();

        let nb = str::parse::<i32>(&caps["nb"]).unwrap();
        let from = str::parse::<i32>(&caps["from"]).unwrap();
        let to = str::parse::<i32>(&caps["to"]).unwrap();
        let mut grabber = Vec::<String>::new();

        {
            let from_crate = &mut (crates[(from - 1) as usize]);
            for _i in 0..nb {
                let val = from_crate.pop_front().unwrap();
                grabber.push(val);
            }
        }

        {
            let to_crate = &mut (crates[(to - 1) as usize]);
            for _i in 0..nb {
                let val = grabber.pop().unwrap();
                to_crate.push_front(val);
            }
        }
    }

    let mut result = "".to_owned();
    for crate_vec in crates {
        result.push_str(crate_vec.front().unwrap());
    }

    println!("Result: {}", result)
}

fn main() {
    let now = Instant::now();
    part2();
    println!("Time elapsed is: {:?}", now.elapsed());
}