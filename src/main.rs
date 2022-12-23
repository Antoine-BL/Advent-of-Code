use std::{fs, collections::HashSet, hash::Hash};
use std::time::{Duration, Instant};

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

        acc += letters_in_both_st.iter().fold(0, |acc, x| acc + x)
    }

    println!("Score: {}", acc)
}

fn part2() {
    let file_path: String = String::from(".\\data\\03");
    let file_content = fs::read_to_string(file_path)
        .expect("File doesn't exist");
    let mut split_file_content = file_content.lines().peekable();

    let mut score = 0;

    let mut priorities = [0u8;256];
    //a-z priority is 1-26
    for i in 1u8..27u8 {
        let index = i + 96u8;
        priorities[index as usize] = i;
    }
    //A-Z priority is 27-53
    for i in 1u8..27u8 {
        let index = i + 64u8;
        priorities[index as usize] = i + 26;
    }

    loop {
        let sack1 = split_file_content.next().unwrap().as_bytes();
        let sack2 = split_file_content.next().unwrap().as_bytes();
        let sack3 = split_file_content.next().unwrap().as_bytes();

        let sack1Letters = get_letters(sack1, &priorities);
        let sack2Letters = get_letters(sack2, &priorities);
        let sack3Letters = get_letters(sack3, &priorities);

        let final_letters = sack1Letters & sack2Letters & sack3Letters;

        let mut cur_score = 0;
        for i in 0..53 {
            cur_score += (((1u64 << i) & final_letters) >> i) * i;
        }
        score += cur_score;
        
        if split_file_content.peek().is_none() {
            break;
        }
    }

    println!("Score: {}", score)
}

fn get_letters(line: &[u8], priorities: &[u8; 256]) -> u64 {
    let mut letters = 0u64;
    for letter in line {
        let priority = priorities[*letter as usize];
        letters = letters | (1 << priority);
    }
    return letters;
}

fn main() {
    let now = Instant::now();
    part2();
    println!("Time elapsed is: {:?}", now.elapsed());
}