use std::env;
use std::fs;
use std::io::{self, Read};
use std::process;

fn print_usage() {
    println!(
        "
srch
Usage:
    srch [OPTIONS] QUERY [FILENAME]
"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        process::exit(0);
    }

    if args.len() < 3 {
        let query = &args[1];
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handler = stdin.lock();

        handler.read_to_string(&mut buffer).unwrap();
        if buffer.len() == 0 {
            println!("empty file");
            process::exit(0);
        }

        let matches: Vec<&str> = buffer
            .split("\n")
            .filter(|line| line.contains(query))
            .collect();
        println!("{matches:#?}");

        process::exit(0);
    }

    let query = &args[1];
    let file_path = &args[2];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let matches: Vec<&str> = contents
        .split("\n")
        .filter(|line| line.contains(query))
        .collect();

    dbg!(matches);
}
