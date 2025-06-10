use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn main() {
    generate_workout(10, 10)
}

struct Cacher<T>
where
    T: Fn(u32) -> u32
{
    calculation: T,
    map: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where 
    T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            map: HashMap::new(),
        }
    }
    
    fn value(&mut self, arg: u32) -> u32 {
        match self.map.get(&arg) { 
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.map.insert(v, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut cached_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today do {} pushups",
            cached_result.value(intensity)
        );
        println!(
            "Next, do {} situps",
            cached_result.value(8)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {}  minutes",
                cached_result.value(intensity)
            );
        }
    }
}