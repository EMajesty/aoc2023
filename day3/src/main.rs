use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let path = Path::new("input");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Couldn't read {}: {}", display, why),
        Ok(_) => {
            let mut sum: u32 = 0;
            let lines = s.lines();
            let mut row_vec: Vec<Vec<char>> = Vec::new();
            //let symbols = ['@', '#', '$', '%', '&', '*', '/', '=', '+'];

            for line in lines {
                let line_vec: Vec<char> = line.chars().collect();
                row_vec.push(line_vec);
            }

            let i_len = row_vec.len();

            for i in 0..i_len {
                let mut number: Vec<u32> = Vec::new();
                let mut is_part: bool = false;
                let mut part_num: u32;

                let j_len = row_vec[i].len();

                for j in 0..j_len {
                    let val = row_vec[i][j];

                    if val.is_numeric() { 
                        number.push(val.to_digit(10).unwrap());

                        if i > 0 && j > 0 && !row_vec[i-1][j-1].is_alphanumeric() && row_vec[i-1][j-1] != '.' { is_part = true; }
                        if i > 0 && !row_vec[i-1][j].is_alphanumeric() && row_vec[i-1][j] != '.' { is_part = true; }
                        if i > 0 && j < j_len - 1 && !row_vec[i-1][j+1].is_alphanumeric() && row_vec[i-1][j+1] != '.' { is_part = true; }
                        if j > 0 && !row_vec[i][j-1].is_alphanumeric() && row_vec[i][j-1] != '.' { is_part = true; }
                        if j < j_len - 1 && !row_vec[i][j+1].is_alphanumeric() && row_vec[i][j+1] != '.' { is_part = true; }
                        if i < i_len - 1 && j > 0 && !row_vec[i+1][j-1].is_alphanumeric() && row_vec[i+1][j-1] != '.' { is_part = true; }
                        if i < i_len - 1 && !row_vec[i+1][j].is_alphanumeric() && row_vec[i+1][j] != '.' { is_part = true; }
                        if i < i_len - 1 && j < j_len - 1 && !row_vec[i+1][j+1].is_alphanumeric() && row_vec[i+1][j+1] != '.' { is_part = true; }

                    } else {
                        if is_part {
                            part_num = vec_to_int(&number);
                            sum += part_num;
                            is_part = false;
                        }

                        number.clear();
                    }
                }
            }
            
            println!("Sum: {sum}");
        },
    }
}

fn vec_to_int(vector: &Vec<u32>) -> u32 {
    let mut result = 0;

    for digit in vector {
        result = result * 10 + digit;
    }

    result
}
