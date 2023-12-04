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

            for line in lines {
                let card: Vec<&str> = line.split(":").collect();
                let lists: Vec<&str> = card[1].split("|").collect();
                let winning_nums: Vec<&str> = lists[0]
                    .trim_start()
                    .trim_end()
                    .split(" ")
                    .collect();
                let your_nums: Vec<&str> = lists[1]
                    .trim_start()
                    .trim_end()
                    .split(" ")
                    .collect();
                let mut card_points: u32 = 0;
                println!("{:?}", card);

                for num in &winning_nums {
                    //assert_eq!(num.chars().count(), 2);
                    if num != &"" {
                        for your_num in &your_nums {
                            //assert_eq!(your_num.chars().count(), 2);
                            if your_num != &"" {
                                if your_num == num {
                                    match card_points {
                                        0 => card_points += 1,
                                        _ => card_points *= 2,
                                    }
                                }
                            }
                        }
                    }
                }

                sum += card_points;
            }

            println!("Sum: {sum}");
        },
    }
}
