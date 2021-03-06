mod chain;

use chain::ChainBuilder;
use structopt::StructOpt;
use std::error::Error;
use std::io::{self, BufRead};

#[derive(Debug, StructOpt)]
struct Opt {
    /// Number of prefix words.
    ///
    /// Longer prefix gives more coherence, but less variability.
    #[structopt(long, default_value = "2")]
    prefix: usize,

    /// Maximum number of words to generate
    #[structopt(default_value = "100")]
    words: usize,
}

fn should_ignore(line :&str) -> bool {
    match line.get(..4) {
        Some("    ") => true,
        Some("-   ") => true,
        _ => false
    }
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    let mut builder = ChainBuilder::new(opt.prefix);
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;

        if should_ignore(&line) {
            continue;
        }

        for word in line.split_whitespace() {
            builder.add(&word);
        }
    }

    let chain = builder.build();
    println!("{}", chain.generate(opt.words).join(" "));

    Ok(())
}
