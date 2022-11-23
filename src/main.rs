#![allow(bad_style)]

// Unfortunately, you currently have to import all four of these.
// We're considering what it would look like to make this redundant,
// and then you'd only need pest and pest-ast.

#[macro_use]
extern crate pest_derive;
extern crate from_pest;
#[macro_use]
extern crate pest_ast;
extern crate pest;

mod csv {
    #[derive(Parser)]
    #[grammar = "./csv_grammar.pest"]
    pub struct Parser;
}

mod ast {
    use super::csv::Rule;
    use pest::Span;

    fn span_into_str(span: Span) -> &str {
        span.as_str()
    }

    #[derive(Debug, FromPest)]
    #[pest_ast(rule(Rule::field))]
    pub struct Field {
        #[pest_ast(outer(with(span_into_str), with(str::parse), with(Result::unwrap)))]
        pub value: f64,
    }

    #[derive(Debug, FromPest)]
    #[pest_ast(rule(Rule::record))]
    pub struct Record {
        pub fields: Vec<Field>,
    }

    #[derive(Debug, FromPest)]
    #[pest_ast(rule(Rule::file))]
    pub struct File {
        pub records: Vec<Record>,
        eoi: EOI,
    }

    #[derive(Debug, FromPest)]
    #[pest_ast(rule(Rule::EOI))]
    struct EOI;
}
/* mod abc_parse;
use clap::{Error, Parser};

/// Simple program to greet a person
#[derive(clap::Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    path: std::path::PathBuf,
}
 */
fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*     let args = Args::parse();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");
    let myresult = abc_parse::abc_parser(&content[..]);
    println!("{:?}", myresult); */

    use ast::File;
    use from_pest::FromPest;
    use pest::Parser;
    use std::fs;

    let source = String::from_utf8(fs::read("./example_tunes/examplecsv.csv")?)?;
    let mut parse_tree = csv::Parser::parse(csv::Rule::file, &source)?;
    println!("parse tree = {:#?}", parse_tree);
    let syntax_tree: File = File::from_pest(&mut parse_tree).expect("infallible");
    println!("syntax tree = {:#?}", syntax_tree);
    println!();

    let mut field_sum = 0.0;
    let mut record_count = 0;

    for record in syntax_tree.records {
        record_count += 1;
        for field in record.fields {
            field_sum += field.value;
        }
    }

    println!("Sum of fields: {}", field_sum);
    println!("Number of records: {}", record_count);

    Ok(())
}

#[test]
fn csv_example_runs() {
    main().unwrap()
}
