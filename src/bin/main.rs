use markov::{
    get_validate_args,
    Command,
    fit, gen, DUMP_FILE,
};

fn main() {
    let command = get_validate_args();
    match command {
        Command::Fit(path) => {
            let chain = fit(&path);
            chain.dump(DUMP_FILE);
            println!("The chain is saved to {}", DUMP_FILE);
        },
        Command::Gen(max_len) => {
            let word = gen(max_len);
            println!("{}", word);
        }
    }
}