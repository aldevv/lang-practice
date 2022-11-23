use std::{env, fs};

pub fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);
    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn load_file(&self) -> Result<String, String> {
        let contents =
            fs::read_to_string(self.filename).expect("Something went wrong reading the file");
        Ok(contents)
    }
}
