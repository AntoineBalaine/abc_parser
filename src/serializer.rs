extern crate derive_more;

use nom::AsBytes;
use nom_locate::LocatedSpan;
use serde::{Deserialize, Serialize};

extern crate serde;

#[derive(Serialize, PartialEq, Eq)]
#[serde(remote = "LocatedSpan")]
struct RemoteSpanDef<T: AsBytes + Serialize> {
    #[serde(getter = "LocatedSpan::location_offset")]
    offset: usize,
    #[serde(getter = "LocatedSpan::location_line")]
    line: u32,
    #[serde(getter = "LocatedSpan::fragment")]
    fragment: T,
    #[serde(getter = "LocatedSpan::get_column")]
    column: usize,
}

// Provide a conversion to construct the remote type.
impl<T: AsBytes + Serialize> From<RemoteSpanDef<T>> for LocatedSpan<T> {
    fn from(def: RemoteSpanDef<T>) -> LocatedSpan<T> {
        LocatedSpan::new(def.fragment)
    }
}

#[derive(serde::Serialize)]
struct Span<T: AsBytes + Serialize>(#[serde(with = "RemoteSpanDef")] LocatedSpan<T>);

#[test]
fn serialize_located_span() {
    let input = Span(LocatedSpan::new(""));
    let serialized = serde_json::to_string(&input).unwrap();
    let converted = "{\"offset\":0,\"line\":1,\"fragment\":\"\",\"column\":1}";
    assert_eq!(serialized, converted)
}
