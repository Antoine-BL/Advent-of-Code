use std::{fs, collections::{BTreeSet, HashSet}, time::Instant};


fn part1() {
    let file_path: String = String::from(".\\data\\06");
    let file_content = fs::read_to_string(file_path)
        .expect("File doesn't exist");

    let mut seen_chars = [' '; 3];
    let mut pos_result = 0;
    let mut unique_counter = 0;
    for (pos, char) in file_content.chars().enumerate() {
        if seen_chars.contains(&char) {
            unique_counter = 0;
            for i in 0..seen_chars.len() {
                seen_chars[i] = ' ';
            }
        } else {
            if unique_counter == 3 {
                pos_result = pos;
                break;
            }
            seen_chars[unique_counter] = char;
            unique_counter += 1;
        }
    }

    println!("{}", pos_result);
}

fn part2() {
    let file_path: String = String::from(".\\data\\06");
    let file_content = fs::read_to_string(file_path)
        .expect("File doesn't exist");

    let mut seen_chars = [' '; 13];
    let mut pos_result = 0;
    let mut unique_counter = 0;
    for (pos, char) in file_content.chars().enumerate() {
        if seen_chars.contains(&char) {
            unique_counter = 0;
            for i in 0..seen_chars.len() {
                seen_chars[i] = ' ';
            }
        } else {
            if unique_counter == 13 {
                pos_result = pos;
                break;
            }
            seen_chars[unique_counter] = char;
            unique_counter += 1;
        }
    }

    println!("{}", pos_result);
}

fn main() {
    let now = Instant::now();
    part2();
    println!("Time elapsed is: {:?}", now.elapsed());
}