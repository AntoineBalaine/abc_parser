use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    multi::many1,
    sequence::tuple,
    IResult,
};

#[derive(Debug, Eq, PartialEq)]
pub struct Pitch<'a> {
    pub alteration: Option<&'a str>,
    pub note_letter: &'a str,
    pub octave: Option<Vec<&'a str>>,
}

impl<'a> Pitch<'a> {
    fn parse_pitch(input: &'a str) -> IResult<&str, Self> {
        let parser = tuple((
            opt(Pitch::parse_alteration),
            Pitch::parse_note_letter,
            opt(Pitch::parse_octave),
        ));
        map(parser, |(alteration, note_letter, octave)| Pitch {
            alteration,
            note_letter,
            octave,
        })(input)
    }

    fn parse_alteration(input: &str) -> IResult<&str, &str> {
        alt((
            tag("="),
            tag("__"),
            tag("_"),
            tag("^^"),
            tag("^"),
            tag("♭"),
            tag("♮"),
            tag("♯"),
            tag("𝄫"),
            tag("𝄪"),
        ))(input)
    }

    fn parse_octave(input: &str) -> IResult<&str, Vec<&str>> {
        many1(alt((tag("'"), tag(","))))(input)
    }

    fn parse_note_letter(input: &str) -> IResult<&str, &str> {
        alt((
            tag("a"),
            tag("b"),
            tag("c"),
            tag("d"),
            tag("e"),
            tag("f"),
            tag("g"),
            tag("A"),
            tag("B"),
            tag("C"),
            tag("D"),
            tag("E"),
            tag("F"),
            tag("G"),
        ))(input)
    }
}

#[test]
fn test_parse_alteration() {
    let test_alterations = [
        ("^^A,", "A,", "^^"),
        ("__B,", "B,", "__"),
        ("_B,", "B,", "_"),
    ];
    for test in test_alterations {
        let (tail, result_alteration) = Pitch::parse_alteration(test.0).unwrap();
        assert_eq!(result_alteration, test.2);
        assert_eq!(tail, test.1);
    }
}

#[test]
fn test_parse_octave() {
    let test_vec = [(",,,", "", [",", ",", ","]), ("'''", "", ["'", "'", "'"])];
    for test in test_vec {
        let (tail, result_octave) = Pitch::parse_octave(test.0).unwrap();
        assert_eq!(result_octave, test.2);
        assert_eq!(tail, test.1);
    }

    let (tail, result_octave) = Pitch::parse_octave("").unwrap();
    assert_eq!(result_octave, Vec::<&str>::new());
    assert_eq!(tail, "");
}

#[test]
fn test_parse_note_letter() {
    let (tail, note_letter) = Pitch::parse_note_letter("G").unwrap();
    assert_eq!(note_letter, "G");
}

#[test]
fn test_parse_pitch() {
    let (tail, pitch) = Pitch::parse_pitch("^G,").unwrap();
    assert_eq!(
        pitch,
        Pitch {
            note_letter: "G",
            alteration: Some("^"),
            octave: Some(vec![","]),
        }
    );
    let (tail, pitch) = Pitch::parse_pitch("G").unwrap();
    assert_eq!(
        pitch,
        Pitch {
            note_letter: "G",
            alteration: None,
            octave: None,
        }
    );
}
