use rand::Rng;
use std::time::{Duration, SystemTime};

const LENGTH: usize = 1_000;
type Arr = [u32; LENGTH];

fn main() {
    let mut rng = rand::thread_rng();
    let tab = [rng.gen::<u32>(); LENGTH];

    // do a few passes first to take advantage of cache
    increment_by_index(tab, 10);
    increment_by_reference(tab, 10);

    let repeats = 10_000;

    let inc_index = increment_by_index(tab, repeats);
    let inc_ref = increment_by_reference(tab, repeats);

    println!("Increment: {:?} {:?}", inc_index, inc_ref)
}

fn increment_by_reference(mut arr: Arr, repeats: u32) -> Duration {
    let start = SystemTime::now();
    for _ in 0..repeats {
        for i in &mut arr {
            *i += 1;
        }
    }
    time_since(start)
}

fn increment_by_index(mut arr: Arr, repeats: u32) -> Duration {
    let start = SystemTime::now();
    for _ in 0..repeats {
        for i in 0..arr.len() {
            arr[i] += 1;
        }
    }

    time_since(start)
}

fn time_since(since: SystemTime) -> Duration {
    let now = SystemTime::now();
    now.duration_since(since).expect("time went backwards")
}
