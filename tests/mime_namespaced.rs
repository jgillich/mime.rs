extern crate mime;

use std::str::FromStr;
use mime::{TopLevel, SubLevel, Attr, Value, Mime};

#[test]
fn parse_text_plain() {
    let mime: Mime = FromStr::from_str("text/plain;charset=utf-8").unwrap();
    assert_eq!(mime, Mime(TopLevel::Text, SubLevel::Plain, vec![(Attr::Charset, Value::Utf8)]));
}
