#[allow(unused_imports)]
use std::io::prelude::*;
use std::process;
use std::{env, fs::File, io::Read};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let mut f = File::open(config.filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong");

    println!("contents:{}", contents);
}

#[allow(dead_code)]
struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough args");
        }
        let (query, filename) = (args[1].clone(), args[2].clone());
        Ok(Config { query, filename })
    }
}
