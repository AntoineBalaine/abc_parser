use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit0;
use nom::multi::many1;
use nom::sequence::pair;
use nom::{
    combinator::{map, opt},
    error::{Error, ErrorKind},
    sequence::tuple,
    Err, IResult,
};
use nom_locate::LocatedSpan;

#[derive(Debug, Eq, PartialEq)]
pub struct Pitch<'a> {
    pub alteration: Option<LocatedSpan<&'a str>>,
    pub note_letter: LocatedSpan<&'a str>,
    pub octave: Option<Vec<LocatedSpan<&'a str>>>,
}

fn prs_rest(input: LocatedSpan<&str>) -> IResult<LocatedSpan<&str>, LocatedSpan<&str>> {
    alt((tag("z"), tag("x")))(input)
}
fn prs_multimeasure_rest(
    input: LocatedSpan<&str>,
) -> IResult<LocatedSpan<&str>, (LocatedSpan<&str>, LocatedSpan<&str>)> {
    pair(alt((tag("Z"), tag("X"))), digit0)(input)
}

#[test]
fn test_prs_rest() {
    let input = LocatedSpan::new("zzzz");
    let (tail, matched_letter) = prs_rest(input).unwrap();
    assert_eq!(matched_letter.fragment(), &"z");
    assert_eq!(tail.fragment(), &"zzz");
}
#[test]
fn test_prs_multimeasure_rest() {
    let (tail, matched) = prs_multimeasure_rest(LocatedSpan::new("Z123abc")).unwrap();
    let (z, digits) = matched;
    assert_eq!(z.fragment(), &"Z");
    assert_eq!(digits.fragment(), &"123");
    assert_eq!(tail.fragment(), &"abc");
}

//rest = { "z" }
//multimeasure_rest = ${ "Z" ~ ASCII_DIGIT* }
impl<'a> Pitch<'a> {
    fn parse_pitch(input: LocatedSpan<&'a str>) -> IResult<LocatedSpan<&'a str>, Self> {
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

    fn parse_octave(
        input: LocatedSpan<&'a str>,
    ) -> IResult<LocatedSpan<&'a str>, Vec<LocatedSpan<&'a str>>> {
        many1(alt((tag("'"), tag(","))))(input)
    }

    fn parse_note_letter(
        input: LocatedSpan<&'a str>,
    ) -> IResult<LocatedSpan<&'a str>, LocatedSpan<&'a str>> {
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

    fn parse_alteration(
        input: LocatedSpan<&'a str>,
    ) -> IResult<LocatedSpan<&'a str>, LocatedSpan<&'a str>> {
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
    let input = LocatedSpan::new("Ablabla");
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
        let (tail, result_alteration) = Pitch::parse_alteration(LocatedSpan::new(test.0)).unwrap();
        assert_eq!(result_alteration.fragment(), &test.2);
        assert_eq!(tail.fragment(), &test.1);
    }
}

#[test]
fn test_parse_octave() {
    let test_vec = [(
        LocatedSpan::new(",,,"),
        LocatedSpan::new(""),
        vec![LocatedSpan::new(","); 3],
    )];
    for test in test_vec {
        let (tail, result_octave) = Pitch::parse_octave(test.0).unwrap();

        for (i, result_span) in result_octave.iter().enumerate() {
            assert_eq!(result_span.fragment(), test.2[i].fragment());
        }
        assert_eq!(tail.fragment(), test.1.fragment());
    }

    let result_octave = Pitch::parse_octave(LocatedSpan::new(""));

    assert_eq!(
        result_octave,
        Err(Err::Error(Error::new(LocatedSpan::new(""), ErrorKind::Tag)))
    );
}

#[test]
fn test_parse_pitch() {
    let (tail, pitch) = Pitch::parse_pitch(LocatedSpan::new("^G,")).unwrap();
    assert_eq!(
        SimplifiedPitch::convert_from_Pitch(&pitch),
        SimplifiedPitch {
            note_letter: "G",
            alteration: Some("^"),
            octave: Some(vec![","]),
        }
    );
    let (tail, pitch) = Pitch::parse_pitch(LocatedSpan::new("G")).unwrap();
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
