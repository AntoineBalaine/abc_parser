use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::multi::many1;
use nom::{
    combinator::{map, opt},
    error::{Error, ErrorKind},
    sequence::tuple,
    Err, IResult,
};
use nom_locate::LocatedSpan;

use crate::serializer::Span;

#[derive(Debug, Eq, PartialEq)]
pub struct Pitch<'a> {
    pub alteration: Option<Span<'a>>,
    pub note_letter: Span<'a>,
    pub octave: Option<Vec<Span<'a>>>,
}

impl<'a> Pitch<'a> {
    fn parse_pitch(input: Span<'a>) -> IResult<Span<'a>, Self> {
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

    fn parse_octave(input: Span<'a>) -> IResult<Span<'a>, Vec<Span<'a>>> {
        many1(alt((tag("'"), tag(","))))(input)
    }

    fn parse_note_letter(input: Span<'a>) -> IResult<Span<'a>, Span<'a>> {
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

    fn parse_alteration(input: Span<'a>) -> IResult<Span<'a>, Span<'a>> {
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
}

#[test]
fn test_parse_note_letter() {
    let input = Span::new("Ablabla");
    let (tail, matched_letter) = Pitch::parse_note_letter(input).unwrap();
    assert_eq!(matched_letter.location_offset(), 0);
    assert_eq!(matched_letter.location_line(), 1);
    assert_eq!(matched_letter.fragment(), &"A");
    assert_eq!(matched_letter.get_column(), 1);
    assert_eq!(tail.fragment(), &"blabla");
}

#[test]
fn test_parse_alteration() {
    let test_alterations = [
        ("^^A,", "A,", "^^"),
        ("__B,", "B,", "__"),
        ("_B,", "B,", "_"),
    ];
    for test in test_alterations {
        let (tail, result_alteration) = Pitch::parse_alteration(Span::new(test.0)).unwrap();
        assert_eq!(result_alteration.fragment(), &test.2);
        assert_eq!(tail.fragment(), &test.1);
    }
}

#[test]
fn test_parse_octave() {
    let test_vec = [(Span::new(",,,"), Span::new(""), vec![Span::new(","); 3])];
    for test in test_vec {
        let (tail, result_octave) = Pitch::parse_octave(test.0).unwrap();

        for (i, result_span) in result_octave.iter().enumerate() {
            assert_eq!(result_span.fragment(), test.2[i].fragment());
        }
        assert_eq!(tail.fragment(), test.1.fragment());
    }

    let result_octave = Pitch::parse_octave(Span::new(""));

    assert_eq!(
        result_octave,
        Err(Err::Error(Error::new(Span::new(""), ErrorKind::Tag)))
    );
}

#[test]
fn test_parse_pitch() {
    let (tail, pitch) = Pitch::parse_pitch(Span::new("^G,")).unwrap();
    assert_eq!(
        SimplifiedPitch::convert_from_Pitch(&pitch),
        SimplifiedPitch {
            note_letter: "G",
            alteration: Some("^"),
            octave: Some(vec![","]),
        }
    );
    let (tail, pitch) = Pitch::parse_pitch(Span::new("G")).unwrap();
    assert_eq!(
        SimplifiedPitch::convert_from_Pitch(&pitch),
        SimplifiedPitch {
            note_letter: "G",
            alteration: None,
            octave: None,
        }
    );
}

#[derive(Debug, Eq, PartialEq)]
struct SimplifiedPitch<'a> {
    pub alteration: Option<&'a str>,
    pub note_letter: &'a str,
    pub octave: Option<Vec<&'a str>>,
}
impl<'a> SimplifiedPitch<'a> {
    fn new_empty() -> Self {
        SimplifiedPitch {
            alteration: None,
            note_letter: "",
            octave: None,
        }
    }
    fn convert_from_Pitch(pitch: &'a Pitch) -> Self {
        let mut simple_pitch = SimplifiedPitch::new_empty();
        match pitch.alteration {
            Some(val) => simple_pitch.alteration = Some(*val.fragment()),
            None => {}
        }
        simple_pitch.note_letter = pitch.note_letter.fragment();
        match pitch.octave.clone() {
            Some(val) => {
                let tester = val.iter().map(|value| *value.fragment()).collect();
                simple_pitch.octave = Some(tester)
            }
            None => {}
        }
        simple_pitch
    }
}

#[test]
fn test_parse_simplifiedpitch() {
    assert_eq!(
        SimplifiedPitch::new_empty(),
        SimplifiedPitch {
            note_letter: "",
            alteration: None,
            octave: None,
        }
    )
}
