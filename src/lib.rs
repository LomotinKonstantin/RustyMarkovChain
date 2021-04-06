use std::path::Path;

extern crate clap;
use clap::{Arg, App, SubCommand};

pub mod linalg;
mod markov_chain;

pub enum Param {
    StrVal(String),
    USizeVal(usize),
}

pub struct Config {
    command: String,
    param: Param,
}

pub fn get_validate_args() -> Config {
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
                        .about("generate a word with specified max length")
                        .arg(Arg::with_name("max_len")
                            .required(true)
                            .help("integer > 0, the maximum length of the generated word")))
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
            Config{
                command: String::from("fit"),
                param: Param::StrVal(String::from(path)),
            }
        },
        // Prediction
        ("gen", Some(val_m)) => {
            let max_len = val_m.value_of("max_len").unwrap().parse::<usize>().unwrap_or_else(|val| {
                println!("Max len must be a positive int. Error: {}", val);
                std::process::exit(0);
            });
            Config{
                command: String::from("gen"),
                param: Param::USizeVal(max_len),
            }
        },
        (&_, _) => {
            println!("Invalid command. Check help");
            std::process::exit(0);
        }
    }
}