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
            let lines = s.lines();
            let mut sum: u32 = 0;
            let mut power_sum: u32 = 0;

            const MAX_RED: u32 = 12;
            const MAX_GREEN: u32 = 13;
            const MAX_BLUE: u32 = 14;

            for line in lines {
                let game: Vec<&str> = line.split(":").collect();
                let game_num = game[0].trim_start_matches("Game ").parse::<u32>().unwrap();
                let rounds: Vec<&str> = game[1].split(";").collect();
                let mut valid_game: bool = true;

                let mut largest_red: u32 = 0;
                let mut largest_green: u32 = 0;
                let mut largest_blue: u32 = 0;

                for round in rounds {
                    let colors: Vec<&str> = round.split(",").collect();

                    for color in colors {
                        let amounts: Vec<&str> = color.trim().split(" ").collect();
                        let amount = amounts[0].parse::<u32>().unwrap();

                        match amounts[1] {
                            "red" => {
                                if amount > largest_red { largest_red = amount; }
                                if amount > MAX_RED { valid_game = false; }
                            },
                            "green" => {
                                if amount > largest_green { largest_green = amount; }
                                if amount > MAX_GREEN { valid_game = false; }
                            },
                            "blue" => {
                                if amount > largest_blue { largest_blue = amount; }
                                if amount > MAX_BLUE { valid_game = false; }
                            },
                            _ => println!("Color \"{}\" does not exist", amounts[1]),
                        }

                        //println!("{:?} {}", amounts, sum);
                    }
                }

                power_sum += largest_red * largest_green * largest_blue;

                if valid_game == true {
                    sum += game_num;
                }
            }

            println!("Sum: {sum}");
            println!("Power sum: {power_sum}");
        },
    }
}
