extern crate swap_cache;
use swap_cache::SwapCache;

extern crate lru;
use lru::LruCache;

extern crate rand;
use rand::Rng;
use rand::distributions::{Distribution, Uniform, LogNormal};

fn test_uniform() {
    let mut cache = SwapCache::new(100);

    let mut rng = rand::thread_rng();
    let normal = Uniform::new(0, 1000);

    let mut keys = Vec::new();

    for i in 0..1000 {
        let k = normal.sample(&mut rng);
        keys.push(k);
    }

    for key in keys.clone() {
        cache.put(key, "some value to store".to_owned());
    }

    let mut hits: f64 = 0.0;
    
    for key in keys.clone() {
        match cache.get(key) {
            Some(_) => hits += 1.0,
            None => (),
        }
    }

    println!("Hits {}", hits/( keys.len() as f64));
}


pub fn main() {
    test_uniform();
}
