use pest::iterators::Pair;
use pest::{error::Error, Parser};

#[derive(Parser, Debug)]
#[grammar = "abc_grammar.pest"]
struct AbcParser;

/// Représente les variantes des règles de la grammaire
#[derive(Debug)]
pub enum AbcValue<'a> {
    TEXTLINE(Vec<AbcValue<'a>>),
    TEXT(&'a str),
}

fn parse_value(pair: Pair<Rule>) -> AbcValue {
    Token::from(pair).kind
}

pub fn abc_parser(song: &str) -> Result<AbcValue, Error<Rule>> {
    let tune = AbcParser::parse(Rule::TEXTLINE, &song)
        .expect("couldn't create the parser")
        .next()
        .unwrap();

    Ok(parse_value(tune))
}

pub struct Token<'a> {
    pub kind: AbcValue<'a>,
    pub line: u32,
    pub col: u32,
    pub end_line: u32,
    pub end_col: u32,
}
// found at https://github.com/pest-parser/pest/issues/276
/// Allow the direction conversion of Pest's `Pair`s into our `Token`s.
/// This removes a lot of logic from walking the `Pair`s.
impl<'i> From<Pair<'i, Rule>> for Token<'i> {
    fn from(pair: Pair<'i, Rule>) -> Self {
        // get the starting position of the token for line / col number;
        // this will get passed all the way through even the AST so that
        // accurate error information can be given even late into assembling
        let span = pair.clone().as_span();
        let start = span.start_pos();
        let (line, col) = start.line_col();
        let end = span.end_pos();
        let (end_line, end_col) = end.line_col();

        Token {
            kind: match pair.as_rule() {
                Rule::TEXTLINE => {
                    let val: Vec<AbcValue> =
                        pair.into_inner().map(|value| parse_value(value)).collect();
                    AbcValue::TEXTLINE(val)
                }
                Rule::TEXT => AbcValue::TEXT(pair.as_str()),
                _ => AbcValue::TEXT(pair.as_str()),
            },
            line: line as u32,
            col: col as u32,
            end_line: end_line as u32,
            end_col: end_col as u32,
        }
    }
}
