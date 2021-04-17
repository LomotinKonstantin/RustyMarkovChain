use std::path::Path;
use std::fs;

extern crate clap;
use clap::{Arg, App, SubCommand};

pub mod linalg;
mod markov_chain;

use markov_chain::TextMarkovChain;

pub const DUMP_FILE: &str = "weights.bin"; 

pub enum Command {
    Fit(String),
    Gen(usize),
}

pub fn get_validate_args() -> Command {
    let matches = App::new("Markov Chain Nickname Generator")
                .version("1.0")
                .author("Konstantin Lomotin <ke.lomotin@gmail.com>")
                .about("Generates random words based on the training set")
                .subcommand(SubCommand::with_name("fit")
                            .about("tune the Markov chain")
                            .arg(Arg::with_name("train_file")
                                .required(true)
                                .help("path to the training file with one word per line")))
                .subcommand(SubCommand::with_name("gen")
                        .about("generate a word with specified minimum length")
                        .arg(Arg::with_name("min_len")
                            .required(true)
                            .help("integer > 0, the minimal length of the generated word")))
    .get_matches();

    match matches.subcommand() {
        // Training
        ("fit", Some(val_m)) => {
            let path = val_m.value_of("train_file").or_else(|| {
                println!("Please specify the path to the training file");
                std::process::exit(0);
            }).unwrap();
            if !Path::new(path).exists() {
                println!("File {} does not exist", path);
                std::process::exit(0);
            };
            Command::Fit(path.to_string())
        },
        // Prediction
        ("gen", Some(val_m)) => {
            let min_len = val_m.value_of("min_len").unwrap().parse::<usize>().unwrap_or_else(|val| {
                println!("Min len must be a positive int. Error: {}", val);
                std::process::exit(0);
            });
            Command::Gen(min_len)
        },
        (&_, _) => {
            println!("Invalid command. Check help");
            std::process::exit(0);
        }
    }
}

pub fn fit(path: &str) -> TextMarkovChain {
    let tr_data = fs::read_to_string(path).expect("Unable to read training file").to_lowercase();
    let tr_data: Vec<&str> = tr_data.split_whitespace().collect();
    TextMarkovChain::fit(&tr_data)
}

pub fn gen(min_len: usize) -> String {
    let mc = TextMarkovChain::load(DUMP_FILE);
    to_titlecase(&mc.gen(min_len))
}

fn to_titlecase(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}