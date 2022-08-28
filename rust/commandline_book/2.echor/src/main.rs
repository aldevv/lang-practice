// requires the clap library
use clap::{App, Arg};
fn main() {
    // primitive and quick way of doing it
    // println!("{:#?}", std::env::args());

    let matches = App::new("echor")
        .version("0.1.0")
        .author("Juan Alejandro Bernal")
        .about("Rust echo")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Input text")
                .takes_value(false),
        )
        .get_matches();

    let text = matches.values_of_lossy("text").unwrap().join(" ");
    let omit_newline = matches.is_present("omit_newline");
    let end_char = if omit_newline { "" } else { "\n" };

    print!("{}{}", text, end_char);
}
