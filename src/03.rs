use std::{fs, collections::HashSet, hash::Hash};

fn get_byte_score(byte_char: &u8) -> i32 {
    let result;
    if byte_char >= &97u8 { // if is a-z
        result = byte_char - 96u8; // a = 1 to z = 26
    } else {
        result = byte_char - 64u8 + 26; // A=27 to Z=53
    };
    return result as i32;
}

fn part1() {
    let file_path: String = String::from(".\\data\\03");
    let file_content = fs::read_to_string(file_path)
        .expect("File doesn't exist");

    let mut acc = 0;
    for line in file_content.split("\n") {
        let line_bytes = line.as_bytes();
        let first_compartment_size = (line_bytes.len() / 2) as i32;
        let mut first_compartment_set = HashSet::<u8>::new();
        let mut letters_in_both_st = HashSet::<u8>::new();

        for i in 0..first_compartment_size {
            first_compartment_set.insert(line_bytes[i as usize]);
        }

        for i in first_compartment_size..line_bytes.len() as i32 {
            if first_compartment_set.contains(&line_bytes[i as usize]) {
                letters_in_both_st.insert(line_bytes[i as usize]);
            }
        }

        acc += letters_in_both_st.iter().fold(0, |acc, x| acc + get_byte_score(x))
    }

    println!("Score: {}", acc)
}

fn part2() {
    let file_path: String = String::from(".\\data\\03");
    let file_content = fs::read_to_string(file_path)
        .expect("File doesn't exist");
    let split_file_content = file_content.split("\n");

    let mut score = 0;
    let mut common_letters = HashSet::<u8>::new();
    let mut cur_letters = HashSet::<u8>::new();
    for (index, line) in split_file_content.enumerate() {
        let line_bytes = line.as_bytes();
        let is_first_in_group = index % 3 == 0;
        let is_last_in_group = index % 3 == 2;
        
        if is_first_in_group {
            common_letters.clear();
            common_letters.extend(line_bytes.iter())
        } else {
            cur_letters.clear();

            cur_letters.extend(line_bytes.iter());
            common_letters.retain(|v| cur_letters.contains(&v));
        }

        if is_last_in_group {
            score += common_letters.iter()
                .fold(0, |acc, x| acc + get_byte_score(x));
        }
    }

    println!("Score: {}", score)
}

fn main() {
    part2();
}