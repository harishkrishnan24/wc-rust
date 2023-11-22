use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 2 {
        panic!("USAGE: wc -c test.txt");
    }

    match args[0].as_str() {
        "-c" => {
            let filename = &args[1];
            let contents = fs::read(filename);
            match contents {
                Ok(data) => println!("{} {}", data.len(), filename),
                Err(e) => panic!("Error reading file: {e}"),
            }
        }
        "-l" => {
            let filename = &args[1];
            let contents = fs::read_to_string(filename);
            match contents {
                Ok(data) => println!("{} {}", data.lines().count(), filename),
                Err(e) => panic!("Error reading file: {e}"),
            }
        }
        "-w" => {
            let filename = &args[1];
            let contents = fs::read_to_string(filename);
            match contents {
                Ok(data) => println!("{} {}", data.split_whitespace().count(), filename),
                Err(e) => panic!("Error reading file: {e}"),
            }
        }
        _ => panic!("invalid arguments"),
    };
}
