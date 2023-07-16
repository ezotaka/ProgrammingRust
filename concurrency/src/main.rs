use rayon::prelude::*;
use std::{
    fmt::{format, Display},
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    sync::{
        mpsc::{self, Receiver},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::Duration,
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_channel();
    //channel_test();
    //channel();
    //    thread::sleep(Duration::from_secs(2));
    Ok(())
}

fn channel_test() {
    let (sender, receiver) = mpsc::channel();

    let sender_clone = sender.clone(); // クローンを作る

    thread::spawn(move || {
        sender.send("Hello, world!").unwrap();
        // このスレッドのスコープからsenderが出ると、このスレッドの所有するSenderはドロップされます。
    });

    thread::spawn(move || {
        sender_clone.send("Another message").unwrap();
        // このスレッドのスコープからsender_cloneが出ると、このスレッドの所有するSenderもドロップされます。
    });

    for message in receiver {
        println!("Received: {}", message);
    }

    println!("All senders have been dropped, and the channel is closed.");
}

fn run_channel() {
    //    ever_print();
    channel2();
}

fn channel() {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        for path in list_files(".").unwrap() {
            let size = fs::metadata(&path).unwrap().len();
            let text = format!("{}\t{}", size, path.display());

            if sender.send(text).is_err() {
                break;
            }
        }
    });
    let handle = print_file(receiver);
    handle.join().unwrap();
}

fn channel2() {
    let (sender, receiver) = mpsc::channel();

    for i in 1..3 {
        let cloned_sender = sender.clone();
        let end = Arc::new(Mutex::new(false));
        let end_clone = end.clone();
        thread::spawn(move || loop {
            if cloned_sender.send(i).is_err() {
                break;
            }

            thread::sleep(Duration::from_millis(10));

            let e = end.lock().unwrap();
            if *e {
                drop(cloned_sender);
                break;
            }
            drop(e);
        });
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(i));
            let mut e = end_clone.lock().unwrap();
            *e = true;
        });
    }
    drop(sender);

    let handle = print_file(receiver);
    handle.join().unwrap();
}

fn list_files<P: AsRef<Path>>(path: P) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry_result in fs::read_dir(path)? {
        let entry = entry_result?;
        let path = entry.path();

        if path.is_dir() {
            files.extend(list_files(&path)?);
        } else {
            files.push(path);
        }
    }

    Ok(files)
}

fn print_file<T: Display + Send + 'static>(receiver: Receiver<T>) -> JoinHandle<()> {
    return thread::spawn(move || {
        for text in receiver {
            println!("{}", text);
            std::io::stdout().flush().unwrap();
        }
        println!("is ended?")
    });
}

fn ever_print() {
    thread::spawn(move || loop {
        println!("###################################################################");
        std::io::stdout().flush().unwrap();
        thread::sleep(Duration::from_micros(1));
    });
}
fn run_threads() {
    let glossary = GigabyteMap::new();
    threads(Arc::new(glossary));
    //thread::sleep(Duration::from_secs(1));
}
fn threads(glossary: Arc<GigabyteMap>) {
    let mut handles = vec![];
    for i in 0..4 {
        let g = glossary.clone();
        handles.push(thread::spawn(move || {
            if i == 2 {
                panic!("{} is panicked", i);
            }
            println!("{} glossary is {:?}", i, g);
            i
        }));
    }

    for handle in handles {
        let result = handle.join();
        match result {
            Ok(i) => println!("finish {}", i),
            Err(e) => eprintln!("error {:?}", e),
        }
    }
}

async fn process() -> Result<(), Box<dyn std::error::Error>> {
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
