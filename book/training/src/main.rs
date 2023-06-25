#![allow(unused)]

use std::{collections::HashMap, hash::Hash};
fn main() {
    use std::thread;
    use std::time::Duration;

    fn generate_workout(intensity: u32, random_number: u32) {
        let mut expensive_result = Cacher::new(|num| {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num
        });

        if intensity < 25 {
            println!(
                // 今日は{}回腕立て伏せをしてください！
                "Today, do {} pushups!",
                expensive_result.value(intensity)
            );

            println!(
                // 次に、{}回腹筋をしてください！
                "Next, do {} situps!",
                expensive_result.value(intensity)
            );
        } else {
            if random_number == 3 {
                // 今日は休憩してください！水分補給を忘れずに！
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!(
                    // 今日は、{}分間走ってください！
                    "Today, run for {} minutes!",
                    expensive_result.value(intensity)
                );
            }
        }
    }
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

struct Cacher<F, K, V>
where
    F: Fn(K) -> V,
{
    calculation: F,
    values: HashMap<K, V>,
}

impl<F, K, V> Cacher<F, K, V>
where
    F: Fn(K) -> V,
    K: Hash + Eq + Clone,
    V: Clone,
{
    fn new(calculation: F) -> Self {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: K) -> V {
        match self.values.get(&arg) {
            Some(v) => v.clone(),
            None => {
                let v = (self.calculation)(arg.clone());
                self.values.insert(arg, v.clone());
                v
            }
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}
