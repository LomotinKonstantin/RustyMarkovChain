mod cli;
mod markov_chain;

use std::fs;

use clap::Parser;
use cli::Cli;
use markov_chain::TextMarkovChain;

use crate::cli::Command;

fn main() {
    use Command as Cmd;

    let args = Cli::parse();
    if let Err(error) = match args.command {
        Cmd::Fit {
            training_list,
            weights_file,
        } => fit(&training_list, &weights_file),
        Cmd::Generate {
            weights_file,
            n_words,
        } => generate(&weights_file, n_words),
    } {
        println!("{error}");
    }
}

fn fit(word_list_file: &str, weights_file: &str) -> anyhow::Result<()> {
    let content = fs::read_to_string(word_list_file)?;
    let mut trainig_data: Vec<&str> = content.split_whitespace().collect();
    trainig_data.sort();
    trainig_data.dedup();
    let chain = TextMarkovChain::fit(&trainig_data);
    chain.save(weights_file)?;
    Ok(())
}

fn generate(weights_file: &str, n_words: u32) -> anyhow::Result<()> {
    let chain = TextMarkovChain::load(weights_file)?;
    for _ in 0..n_words {
        println!("{}", to_titlecase(&chain.gen()));
    }
    Ok(())
}

fn to_titlecase(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
