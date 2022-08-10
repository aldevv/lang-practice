use std::io;

fn count(mut r: impl io::BufRead) -> io::Result<i32> {
    let mut buf = String::new();
    r.read_line(&mut buf)?;
    let mut wc = 0;

    for _ in buf.split(' ') {
        wc += 1;
    }
    Ok(wc)
}

fn main() {
    let res = count(io::stdin().lock());
    println!("{}", res.unwrap());
}

#[test]
fn test_count() {
    let exp = 4;
    let my_str = "hello world nice day";
    let res = count(my_str.as_bytes()).unwrap();
    assert_eq!(res, exp)
}
