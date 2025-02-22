use rand::distributions::{Alphanumeric, DistString};
use std::collections::{HashMap, HashSet};

/// to show the collision rate
fn main() {
    let mut rng = rand::thread_rng();

    let mut data = HashSet::new();
    let mut all_tries = HashMap::from([(4, 0), (5, 0), (6, 0), (7, 0)]);
    let mut collision_tries = HashMap::from([(4, 0), (5, 0), (6, 0), (7, 0)]);

    while all_tries[&4] < 10_000_000 {
        let mut key_len = 4;
        *all_tries.get_mut(&key_len).unwrap() += 1;
        let mut key = Alphanumeric.sample_string(&mut rng, key_len);
        while data.contains(&key) {
            *collision_tries.get_mut(&key_len).unwrap() += 1;
            key_len += 1;
            *all_tries.get_mut(&key_len).unwrap() += 1;
            key = Alphanumeric.sample_string(&mut rng, key_len);
        }
        data.insert(key);

        // print result
        if all_tries[&4] % 1_000_000 == 0 {
            for i in 4..8 {
                let mut collision_rate = 0.0;
                if all_tries[&i] > 0 {
                    collision_rate = collision_tries[&i] as f64 / all_tries[&i] as f64;
                }
                println!(
                    "length {}: {} tries, {} collisions\ncollision rate: {}",
                    i, all_tries[&i], collision_tries[&i], collision_rate
                );
            }
            println!();
        }
    }
}
