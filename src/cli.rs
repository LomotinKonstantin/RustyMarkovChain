use clap::{command, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "Markov Chain Nickname Generator")]
#[command(author = "Konstantin Lomotin <ke.lomotin@gmail.com>")]
#[command(version = "1.1.0")]
#[command(about = "Generates random words based on the training set distribution", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(about = "Tune the Markov chain")]
    Fit {
        #[arg(help = "Path to the list of the words to learn")]
        #[arg(long, short)]
        training_list: String,
        #[arg(help = "Path to the learned weights")]
        #[arg(long, short)]
        weights_file: String,
    },
    #[command(about = "Tune the Markov chain")]
    Generate {
        #[arg(help = "Path to the learned weights")]
        #[arg(long, short)]
        weights_file: String,
        #[arg(help = "Number of words to generate")]
        #[arg(long, short)]
        n_words: u32,
    },
}
