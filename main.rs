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

            


            println!("Sum: {sum}");
        },
    }
}
