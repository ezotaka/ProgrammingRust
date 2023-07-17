fn main() {
    assert_eq!(triangle(4), 10);
    into_iter();
    //from_fn();
    fibonacci();
}

fn triangle(n: i32) -> i32 {
    (1..=n).fold(0, |sum, item| sum + item)
}

fn into_iter() {
    let mut favorites = vec![]; //HashSet::new();
    favorites.insert(0, "Lucy".to_string());
    favorites.insert(0, "Lie".to_string());

    for e in &mut favorites {
        *e += "hoge";
        println!("{}", e);
    }
}

fn from_fn() {
    use rand::random;
    use std::iter::from_fn;

    let lengths: Vec<f64> = from_fn(|| {
        let diff = random::<f64>() - random::<f64>();
        if diff < 0 as f64 {
            None
        } else {
            Some(diff)
        }
    })
    .take(100)
    .collect();

    println!("{:?}", lengths);
}

fn fibonacci() {
    use std::iter::successors;
    let nums: Vec<_> = successors(Some((0, 1)), |acc| Some((acc.1, acc.0 + acc.1)))
        .take(10)
        .collect();

    println!("{:?}", nums);
}
