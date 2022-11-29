use pest::Parser;

#[derive(Parser)]
#[grammar = "abc_grammar.pest"]
struct AbcParser;

pub fn abc_parse(song: String) {
    let tune = AbcParser::parse(Rule::file_structure, &song);
    println!("{:?}\n\n", tune.expect("parsing failed"));
}
