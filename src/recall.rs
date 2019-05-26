extern crate swap_cache;
use swap_cache::SwapCache;

extern crate lru;
use lru::LruCache;

extern crate rand;
//use rand::Rng;
use rand::distributions::{
    Distribution,
    Uniform,
    LogNormal,
    Normal,
    Weibull,
    Triangular,
    Gamma,
};

trait Cache<K,V> where
    K: Clone + std::cmp::Eq + std::hash::Hash
{
    fn get(&mut self, key: &K) -> Option<&V>;
    fn put(&mut self, key: K, value: V);
}

impl<K,V> Cache<K,V> for SwapCache<K,V>  where
    K: Clone + std::cmp::Eq + std::hash::Hash
{
    fn get(&mut self, key: &K) -> Option<&V> {
        SwapCache::get(self, key)
    }
    
    fn put(&mut self, key: K, value: V) {
        SwapCache::put(self, key, value)
    }

}

impl<K,V> Cache<K,V> for LruCache<K,V>  where
    K: Clone + std::cmp::Eq + std::hash::Hash
{
    fn get(&mut self, key: &K) -> Option<&V> {
        LruCache::get(self, key)
    }
    
    fn put(&mut self, key: K, value: V) {
        LruCache::put(self, key, value)
    }

}


fn run_recall<K,V>( cache: &mut Cache<K,V>, data: &Vec<(K,V)>) -> f64 where
    K: Clone + std::cmp::Eq + std::hash::Hash,
    V: Clone,
{
    let mut hits: f64 = 0.0;

    for (key, value) in data[..].iter() {
        match cache.get(key) {
            Some(_) => hits += 1.0,
            None => cache.put(key.clone(), (*value).clone()),
        }
    }

    hits/(data.len() as f64)
}

fn test_uniform() {

    let mut rng = rand::thread_rng();
    let normal = Uniform::new(0, 1000);

    let mut data = Vec::new();

    for _i in 0..1000 {
        let k = normal.sample(&mut rng);
        data.push((k,"some value to store".to_owned()));
    }

    let mut sw = SwapCache::new(100);
    let sw_hits = run_recall(&mut sw, &data);

    let mut lru = LruCache::new(100);
    let lru_hits = run_recall(&mut lru, &data);

    
    println!("Hits uniform SwapCache {} LruCache {}", sw_hits, lru_hits);
}


fn test_distribution<D>(distribution: &D, name: String) where
    D: Distribution<f64>,
{

    let mut rng = rand::thread_rng();

    let mut data = Vec::new();

    for _i in 0..100000 {
        let key = distribution.sample(&mut rng);
        //println!(" sample {}", key.abs()); //BOOG
        data.push((key.abs() as u64, "some value to store".to_owned()));
    }

    let mut sw = SwapCache::new(100);
    let sw_hits = run_recall(&mut sw, &data);

    let mut lru = LruCache::new(100);
    let lru_hits = run_recall(&mut lru, &data);

    
    println!("Hits {} SwapCache {} LruCache {} diff {}", name, sw_hits, lru_hits, sw_hits - lru_hits);
}

fn test_log_normal() {
    test_distribution(&LogNormal::new(2.0, 3.0), "LogNormal".to_owned());
    test_distribution(&Normal::new(2.0, 500.0), "Normal".to_owned());
    test_distribution(&Weibull::new(500., 1.), "Weibull".to_owned());
    test_distribution(&Triangular::new(0., 1000., 2.5), "Triangular".to_owned());
    //BOOG causes panic! test_distribution(&Gamma::new(2.0, 5.0), "Gamma".to_owned());
    test_distribution(&Gamma::new(2.0, 200.0), "Gamma".to_owned());
   
    
}

pub fn main() {
    test_uniform();
    test_log_normal();
}
