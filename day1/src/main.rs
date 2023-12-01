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
