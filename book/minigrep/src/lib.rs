use std::io::prelude::*;
use std::{error::Error, fs::File};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("contents:{}", contents);

    Ok(())
}

#[allow(dead_code)]
pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough args");
        }
        let (query, filename) = (args[1].clone(), args[2].clone());
        Ok(Config { query, filename })
    }
}
