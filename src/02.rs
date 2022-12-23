use std::{fs};

fn main() {
    let file_path: String = String::from(".\\data\\02");
    let file_content = fs::read_to_string(file_path)
        .expect("File doesn't exist");
    let score_for_result = [0, 3, 6];
    let play_for_desired_result = [
        [3, 1, 2], 
        [1, 2, 3], 
        [2, 3, 1]
    ];
    
    let mut acc = 0;
    for line in file_content.split("\n") {
        if line.len() == 0 { continue; }

        let line_bytes = line.as_bytes();
        let elf_play = (line_bytes[0] - 65u8) as i32;
        let my_play = (line_bytes[2] - 88u8) as i32;
        let play_score = score_for_result[my_play as usize];
        let win_score = play_for_desired_result[my_play as usize][elf_play as usize];
        acc += play_score + win_score;
        println!("{}: {}", line, play_score + win_score);
    }
    println!("Score: {}", acc       )
}