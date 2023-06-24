#![allow(unused)]
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

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Self {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
