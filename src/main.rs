mod abc_parse;
use clap::Parser;
extern crate pest;
#[macro_use]
extern crate pest_derive;

/// Simple program to greet a person
#[derive(clap::Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    path: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");
    let myresult = abc_parse::abc_parser(&content[..]);
    println!("{:?}", myresult)
}
