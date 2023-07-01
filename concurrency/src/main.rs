use std::{sync::Arc, thread};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let files = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let mut map = GigabyteMap::new();
    map.insert("a".to_string(), "A".to_string());
    map.insert("b".to_string(), "B".to_string());
    map.insert("c".to_string(), "C".to_string());
    process_file_in_parallel(files, Arc::new(map));
    Ok(())
}

fn process_file_in_parallel(file_names: Vec<String>, glossary: Arc<GigabyteMap>) {
    let mut handles = vec![];
    for file in file_names {
        let glossary_for_child = glossary.clone();
        handles.push(thread::spawn(move || {
            process_files(file, glossary_for_child)
        }))
    }
    for h in handles {
        h.join().unwrap();
    }
}

fn process_files(file_name: String, glossary: Arc<GigabyteMap>) {
    println!(
        "hello from child thread {} {}",
        file_name,
        glossary.get(&file_name).unwrap()
    );
}

type GigabyteMap = std::collections::HashMap<String, String>;
