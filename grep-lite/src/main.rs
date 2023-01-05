use clap::{arg, Command};

use regex::Regex;

use std::collections::VecDeque;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::process::exit;
use std::str::FromStr;

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex, context_lines: usize) {
    let capacity = context_lines * 2 + 1;
    let mut context = VecDeque::with_capacity(capacity);
    // TODO(spike): handle passing context_lines > lines / 2
    let mut counter = context_lines;
    let mut found = false;

    for line_ in reader.lines() {
        if found && counter == 0 {
            for found_line in &context {
                println!("{found_line}");
            }
            exit(0);
        }

        if context.len() == capacity {
            context.pop_front();
        }

        let line = line_.expect("should not be reached if there are no lines");

        context.push_back(line.clone());

        if found {
            counter -= 1;
        }

        if re.find(&line).is_some() {
            found = true;
        }
    }
}

fn main() {
    let args = Command::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(
            arg!(-p --pattern <VALUE>)
                .help("The pattern to search for")
                .required(true),
        )
        .arg(
            arg!(-i --input <VALUE>)
                .help("File to search")
                .required(false),
        )
        .arg(
            arg!(-c --context <VALUE>)
                .help("Context around lines")
                .required(false),
        )
        .get_matches();

    let pattern = args
        .get_one::<String>("pattern")
        .expect("should be required");
    let context_lines = match args.get_one::<String>("context") {
        Some(val) => i32::from_str(val).unwrap() as usize,
        None => 0,
    };

    let re = Regex::new(pattern).expect("should compile");

    if let Some(input) = args.get_one::<String>("input") {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re, context_lines);
    } else {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, re, context_lines);
    }
}
