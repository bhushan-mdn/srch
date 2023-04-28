#![feature(drain_filter)]

use std::env;
use std::fs;
use std::io::stdout;
use std::io::{self, Read};
use std::process;

fn print_usage() {
    println!(
        "
srch
Usage:
    srch [OPTIONS] QUERY [FILENAME]
Options:
    v - invert matches
    n - show line numbers
"
    );
}

fn main() {
    let mut args: Vec<String> = env::args().collect();

    let opts: Vec<String> = args.drain_filter(|arg| arg.starts_with("-")).collect();

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

    let invert_match = opts.contains(&"-v".to_string());

    let matches: Vec<&str> = contents
        .split("\n")
        .filter(|line| {
            if invert_match {
                !line.contains(query)
            } else {
                line.contains(query)
            }
        })
        .collect();

    for m in matches {
        // let output = m.replace(query, &color_red(query));
        println!("{}", m);
    }
}

fn color_red(original: &str) -> String {
    format!("\033[91m{}\033[0m", original)
}
