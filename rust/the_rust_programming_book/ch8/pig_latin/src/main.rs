use std::io;

const VOWELS: [char; 10] = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

fn is_vowel(c: &char) -> bool {
    VOWELS.contains(c)
}

fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    let mut buf = buf.trim().to_string();

    let first = buf.chars().next().unwrap();

    if is_vowel(&first) {
        buf += "-hay";
    } else {
        buf.remove(0);
        buf += &format!("-{}ay", first);
    }

    println!("{}", buf);
    Ok(())
}
