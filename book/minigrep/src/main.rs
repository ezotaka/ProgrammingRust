use std::io::prelude::*;
use std::{env, fs::File, io::Read};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    let mut f = File::open(config.filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong");

    println!("contents:{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let (query, filename) = (args[1].clone(), args[2].clone());
        Config { query, filename }
    }
}
