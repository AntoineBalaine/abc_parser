use nom_locate::LocatedSpan;
use serde::{Deserialize, Serialize};

extern crate serde;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
struct Span<'a> {
    offset: usize,
    line: u32,
    fragment: &'a str,
    column: usize,
}
impl<'a> Span<'a> {
    fn new(input: &'a str) -> Self {
        Span {
            offset: 0,
            line: 1,
            fragment: input,
            column: 1,
        }
    }
}
impl<'a> From<LocatedSpan<&'a str>> for Span<'a> {
    fn from(span: LocatedSpan<&'a str>) -> Self {
        Span {
            offset: span.location_offset(),
            line: span.location_line(),
            fragment: span.fragment(),
            column: span.get_column(),
        }
    }
}

#[test]
fn test_new_span() {
    assert_eq!(
        Span::new(""),
        Span {
            offset: 0,
            line: 1,
            fragment: "",
            column: 1,
        }
    );
}
#[test]
fn test_convert_locatedspan() {
    let input = LocatedSpan::new("Ablabla");
    let converted_span = Span {
        offset: input.location_offset(),
        line: input.location_line(),
        fragment: input.fragment(),
        column: input.get_column(),
    };
    assert_eq!(Span::from(input), converted_span)
}
#[test]
fn test_serialize_span() {
    let input = Span::new("");
    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&input).unwrap();
    let converted = "{\"offset\":0,\"line\":1,\"fragment\":\"\",\"column\":1}";
    assert_eq!(serialized, converted)
}
