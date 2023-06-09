use std::io::prelude::*;
use std::{env, fs::File, io::Read};

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = (&args[1], &args[2]);
    // print query and filename
    println!("{}, {}", query, filename);

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong");

    println!("contents:{}", contents);
}
