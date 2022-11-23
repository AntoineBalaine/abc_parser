use pest_consume::{match_nodes, Error, Parser};

#[derive(Debug)]
pub enum PitchElements<'a> {
    Alteration(&'a str),
    Octave(&'a str),
    NoteLetter(&'a str),
}
type Pitch<'a> = Vec<PitchElements<'a>>;
type ABCFile<'a> = Vec<Pitch<'a>>;

type Result<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

#[derive(Parser)]
#[grammar = "./abc_grammar_wip.pest"]
pub struct ABCParser;

#[pest_consume::parser]
impl ABCParser {
    fn EOI(_input: Node) -> Result<()> {
        Ok(())
    }

    fn note_letter(input: Node) -> Result<PitchElements> {
        Ok(PitchElements::NoteLetter(input.as_str()))
    }

    fn octave(input: Node) -> Result<PitchElements> {
        Ok(PitchElements::Octave(input.as_str()))
    }
    fn alteration(input: Node) -> Result<PitchElements> {
        Ok(PitchElements::Alteration(input.as_str()))
    }

    fn pitch(input: Node) -> Result<Pitch> {
        Ok(match_nodes!(input.into_children();
            [alteration(fields)..] => fields.collect(),
            [octave(fields)..] => fields.collect(),
            [note_letter(fields)..] => fields.collect(),
        ))
    }

    fn file(input: Node) -> Result<ABCFile> {
        Ok(match_nodes!(input.into_children();
            [pitch(records).., _] => records.collect(),
        ))
    }
}

pub fn parse_abc(input_str: &str) -> Result<ABCFile> {
    // Parse the input into `Nodes`
    let inputs = ABCParser::parse(Rule::file, input_str)?;
    // There should be a single root node in the parsed tree
    let input = inputs.single()?;
    // Consume the `Node` recursively into the final value
    ABCParser::file(input)
}
