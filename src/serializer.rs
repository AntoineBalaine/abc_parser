use std::fmt::{Debug, Display};
use std::str;

use std::ops::{Deref, DerefMut};
extern crate derive_more;
use derive_more::{Display, From, Into};

use nom::AsBytes;
use nom_locate::LocatedSpan;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};

extern crate serde;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct Span<'a> {
    offset: usize,
    line: u32,
    fragment: &'a str,
    column: usize,
}
impl<'a> Span<'a> {
    pub fn new(input: &'a str) -> Self {
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

#[derive(Debug, Clone, PartialEq, Eq, From, Into)]
struct MySpan<T: AsBytes>(LocatedSpan<T>);

impl<T: AsBytes> Deref for MySpan<T> {
    type Target = LocatedSpan<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: AsBytes> DerefMut for MySpan<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: AsBytes + Debug + Display> Serialize for MySpan<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_struct("MySpan", 4)?;
        seq.serialize_field("offset", &self.location_offset())?;
        seq.serialize_field("line", &self.location_line())?;
        seq.serialize_field(
            "fragment",
            &str::from_utf8(self.fragment().as_bytes()).unwrap(),
        )?;
        seq.serialize_field("column", &self.get_column())?;
        seq.end()
    }
}

impl<T: AsBytes + Debug> MySpan<T> {
    fn new(input: T) -> Self {
        MySpan(LocatedSpan::new(input))
    }
}

#[test]
fn serialize_my_span() {
    let input = MySpan::new("");
    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&input).unwrap();
    let converted = "{\"offset\":0,\"line\":1,\"fragment\":\"\",\"column\":1}";
    assert_eq!(serialized, converted)
}
