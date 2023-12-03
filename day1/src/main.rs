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
            s = s.replace("one", "one1one");
            s = s.replace("two", "two2two");
            s = s.replace("three", "three3three");
            s = s.replace("four", "four4four");
            s = s.replace("five", "five5five");
            s = s.replace("six", "six6six");
            s = s.replace("seven", "seven7seven");
            s = s.replace("eight", "eight8eight");
            s = s.replace("nine", "nine9nine");
            //s = s.replace("zero", "0");
            s = s.replace(&['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'][..], "");

            let lines = s.lines();
            let mut sum: u32 = 0;

            for line in lines {
                match line.chars().count() {
                    1 => {
                        let line = String::from(line) + line;
                        sum += line.parse::<u32>().unwrap();
                    },
                    2 => {
                        sum += line.parse::<u32>().unwrap();
                    },
                    count => {
                        let first = line.get(0..1).unwrap();
                        let last = line.get((count-1)..count).unwrap();
                        let line = String::from(first) + last;
                        sum += line.parse::<u32>().unwrap();
                    },
                }
            }

            println!("{sum}");
        },
    }
}
