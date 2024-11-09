use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simlulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simlulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Stay hydrated!");
        } else {
            println!(
                "Today, run for {} mins!",
                simlulated_expensive_calculation(intensity)
            );
        }
    }
}

fn simlulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
