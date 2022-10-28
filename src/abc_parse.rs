use pest::{error::Error, Parser};

#[derive(Parser, Debug)]
#[grammar = "abc_grammar.pest"]
struct AbcParser;

#[derive(Debug)]
pub enum AbcValue<'a> {
    TEXTLINE(Vec<AbcValue<'a>>),
    TEXT(&'a str),
}

pub fn abc_parser(song: &str) -> Result<AbcValue, Error<Rule>> {
    let tune = AbcParser::parse(Rule::TEXTLINE, &song)
        .expect("couldn't create the parser")
        .next()
        .unwrap();
    use pest::iterators::Pair;

    fn parse_value(pair: Pair<Rule>) -> AbcValue {
        match pair.as_rule() {
            Rule::TEXTLINE => {
                let val: Vec<AbcValue> =
                    pair.into_inner().map(|value| parse_value(value)).collect();
                AbcValue::TEXTLINE(val)
            }
            Rule::TEXT => AbcValue::TEXT(pair.as_str()),
            _ => AbcValue::TEXT("default"),
        }
    }
    Ok(parse_value(tune))
}
