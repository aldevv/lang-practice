use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[allow(dead_code)]
pub struct Config {
    files: Vec<String>,
    num_lines: bool,
    num_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Juan Alejandro Bernal <jbernalxyz@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("file to concatenate")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("num_lines")
                .short("n")
                .help("number of lines"),
        )
        .arg(
            Arg::with_name("num_nonblank_lines")
                .short("b")
                .help("number of nonblank lines"),
        )
        .get_matches();
    let files = matches.values_of_lossy("file").unwrap();
    let num_lines = matches.is_present("num_lines");
    let num_nonblank_lines = matches.is_present("num_nonblank_lines");

    Ok(Config {
        files,
        num_lines,
        num_nonblank_lines,
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}", err),
            Ok(mut f) => {
                let mut buf = String::new();
                if config.num_lines {
                    let mut count = 1;
                    while f.read_line(&mut buf)? != 0 {
                        print!("{}. {}", count, &buf);
                        buf.clear();
                        count += 1;
                    }
                    continue;
                }
                if config.num_nonblank_lines {
                    let mut count = 1;
                    while f.read_line(&mut buf)? != 0 {
                        if &buf == "\n" {
                            print!("\n");
                        } else {
                            print!("{}. {}", count, &buf);
                            count += 1;
                        }
                        buf.clear()
                    }
                    continue;
                }
                f.read_to_string(&mut buf)?;
                print!("{}", &buf)
            }
        }
    }
    Ok(())
}
