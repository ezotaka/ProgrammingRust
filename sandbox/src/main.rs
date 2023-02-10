fn main() {
    let mut i = 1;
    let ten_millis = std::time::Duration::from_millis(100);
    loop {
        i *= 10;
        println!("{}", i);
        std::thread::sleep(ten_millis);
    }
}
