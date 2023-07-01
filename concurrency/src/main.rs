use rayon::prelude::*;
use std::{fmt::format, io};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let files = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let mut map = GigabyteMap::new();
    map.insert("a".to_string(), "A".to_string());
    map.insert("b".to_string(), "B".to_string());
    map.insert("c".to_string(), "C".to_string());
    println!("{}", process_file_in_parallel(files, &map).unwrap());
    Ok(())
}

fn process_file_in_parallel(
    file_names: Vec<String>,
    glossary: &GigabyteMap,
) -> Result<String, io::Result<()>> {
    file_names
        .par_iter()
        .map(|file_name| process_files(file_name, glossary))
        .reduce_with(|r1, r2| Ok(format!("{} {}", r1.unwrap(), r2.unwrap())))
        .unwrap()
}

fn process_files<'a>(file_name: &str, glossary: &'a GigabyteMap) -> Result<String, io::Result<()>> {
    let res = glossary.get(file_name).unwrap();
    println!("hello from child thread {} {}", file_name, res);
    Ok(res.to_owned())
}

type GigabyteMap = std::collections::HashMap<String, String>;
