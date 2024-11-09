use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

struct Catcher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    values: HashMap<u32, u32>,
}

impl<T> Catcher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Catcher<T> {
        Catcher {
            calculation,
            values: HashMap::new(),
        }
    }

    // TODO: refactor with hashmap
    fn value(&mut self, arg: u32) -> u32 {
        if let Some(&v) = self.values.get(&arg) {
            v
        } else {
            let v = (self.calculation)(arg);
            self.values.insert(arg, v);
            v
        }
    }
}
fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Catcher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Stay hydrated!");
        } else {
            println!("Today, run for {} mins!", expensive_result.value(intensity));
        }
    }
}

fn simlulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

#[test]
fn call_with_different_values() {
    let mut c = Catcher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}
