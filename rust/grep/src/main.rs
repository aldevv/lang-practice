use std::{
    borrow::Cow,
    error::Error,
    io::{self, BufRead},
};

fn grep<'a>(mut r: impl BufRead) -> io::Result<&'a str> {
    let mut buf = String::new();
    r.read_line(&mut buf).unwrap();
    let val = "my val";

    if buf.contains(val) {
        return Ok(val);
    }
    return Ok(buf.as_str());
}

fn main() {
    grep(io::stdin().lock());
}

#[test]
fn test_grep() {
    let exp = "my val";
    let query = "my text
    that has a val that is my val right?";
    assert_eq!(grep(query.as_bytes()), exp)
}
