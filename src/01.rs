use std::{ fs};

fn main() {
    let file_path: String = String::from(".\\data\\01");
    let maxes = count_cals(&file_path);
    println!("Top 3:");
    for i in maxes {
        println!("{}", i)
    }
    let result = maxes.iter().fold(0, |acc, x| acc + x);
    println!("Elf with most calories: {result}")
}

fn count_cals(file_path: &String) -> [i32; 3] {
    let calories_string = fs::read_to_string(file_path)
        .expect("File {filePath} doesn't exist");
    let split_lines = calories_string.split("\n");
    
    let mut maxes = [0; 3];
    let mut cals_acc = 0;
    for line in split_lines {
        if line == "" {
            maxes = update_maxes(maxes, cals_acc);
            cals_acc = 0
        } else {
            let parsed_line = line.parse::<i32>()
                .expect("Invalid {line}");
            cals_acc += parsed_line
        }
    }

    update_maxes(maxes, cals_acc);

    return maxes;
}

fn update_maxes(maxes: [i32; 3], new_value: i32) -> [i32; 3] {
    let mut val = new_value;
    let mut new_maxes = [0;3];
    for i in 0..3 {
        if val > maxes[i] {
            let temp = maxes[i];
            new_maxes[i] = val;
            val = temp;
        } else {
            new_maxes[i] = maxes[i];
        }
    }
    return new_maxes;
}