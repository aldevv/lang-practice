use std::{io::{self, BufRead}};

fn grep (mut r: impl BufRead) -> io::Result<String> {
    let mut buf = String::new();
    r.read_line(&mut buf).unwrap();
    let val = "my val";

    if buf.contains(val) {
        return Ok(String::from(val))
    }
    return Ok(buf)
}

fn main() {
    println!("{}", grep(io::stdin().lock()).unwrap());
}

#[test]
fn test_grep() {
    let exp = String::from("my val");
    let query = "my text that has a val that is my val right?";
    assert_eq!(grep(query.as_bytes()).unwrap(), exp )
}
