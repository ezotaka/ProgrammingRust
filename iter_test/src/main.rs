use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    fmt::Display,
};

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

fn drain() {
    let mut outer = "Earth".to_string();
    let inner = String::from_iter(outer.drain(1..4));

    assert_eq!(outer, "Eh");
    assert_eq!(inner, "art");
}

fn format_iter<I: IntoIterator>(iter: I) -> String
where
    I::Item: Display,
{
    // for i in iter {
    //     print!("{}", i);
    // }
    let s: Vec<String> = iter.into_iter().map(|x| x.to_string()).collect();
    format!("[{}]", s.join(","))
}

macro_rules! pi {
    ($iter:expr) => {
        println!("{:?}", format_iter($iter));
    };
}

macro_rules! pw {
    ($windows:expr) => {
        println!("------------");
        for i in $windows {
            for &j in i {
                print!("{:?}", j);
            }
            println!();
        }
    };
}

fn repr_funcs() {
    pi!(1..10);
    pi!((1..20).step_by(3));
    //    pi!(1..);
    pi!(Some(10).iter());
    // let blah: Result<&str, ()> = Ok("blah").ite;
    // pi!(blah);

    let v: Vec<i32> = (1..10).collect();
    pw!(v.windows(5));
    pw!(v.chunks(3));

    let mut v_mut = (1..10).collect::<Vec<i32>>();
    for w in v_mut.chunks_mut(3) {
        w[0] *= 2;
    }
    println!("{:?}", v_mut);

    pw!(v.split(|i| i % 4 == 0));
    pw!(v.rsplit(|i| i % 4 == 0));
    pw!(v.splitn(2, |i| i % 4 == 0));

    let s = "abc1def2ghij3";
    pi!(s.bytes());
    pi!(s.chars());
    pi!(s.split(|c| c as i8 % 3 == 0));
    pi!(s.matches(|c| c as i8 % 3 == 0));

    let m = (1..5).zip(vec!['a', 'b', 'c', 'd']);
    let hm: HashMap<_, _> = m.clone().collect();
    let mut bm: BTreeMap<_, _> = m.clone().collect();
    println!("{:?}", hm);
    println!("{:?}", bm);
    println!("{:?}", bm.keys());
    println!("{:?}", bm.values());
    for v in bm.values_mut() {
        *v = (*v as u8 + 10) as char;
    }
    println!("{:?}", bm.values());

    let s1 = BTreeSet::from_iter(vec![1, 2, 3]);
    let s2 = BTreeSet::from_iter(vec![2, 3, 4, 5]);
    pi!(s1.union(&s2));
    pi!(s1.intersection(&s2));

    pi!(std::iter::empty::<i32>());
    pi!(std::iter::once(5));
    pi!(std::iter::repeat("hoge").take(5));
}

fn main() {
    // assert_eq!(triangle(4), 10);
    // into_iter();
    // //from_fn();
    // fibonacci();
    // drain();
    repr_funcs();
}
