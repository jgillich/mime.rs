extern crate mime;

use std::str::FromStr;
use mime::{Mime, Text, Plain, Charset, Utf8};

#[test]
fn parse_text_plain() {
    let mime: Mime = FromStr::from_str("text/plain;charset=utf-8").unwrap();
    assert_eq!(mime, Mime(Text, Plain, vec![(Charset, Utf8)]));
}
