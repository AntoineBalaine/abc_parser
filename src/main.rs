pub mod pitch;
pub mod serializer;
//use clap::Parser;

/// Simple program to greet a person
#[derive(clap::Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    path: std::path::PathBuf,
}

fn main() {
    /*     let args = Args::parse();
    let content = std::fs::read_to_string(&args.path).expect("could not read file"); */
    /*     let successful_parse = AbcParser::parse(Rule::TEXT, "-273.15");
    println!("{:?}\n\n", successful_parse.unwrap());

    let unsuccessful_parse = AbcParser::parse(Rule::TEXT, "this is a %");
    println!("{:?}", unsuccessful_parse); */
}
