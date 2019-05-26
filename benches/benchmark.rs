#[macro_use]
extern crate criterion;

use criterion::{Criterion, black_box};

/*
extern crate rand;
use rand::Rng;
use rand::distributions::{Distribution, Standard};
 */

extern crate swap_cache;
use swap_cache::SwapCache;

extern crate lru;
use lru::LruCache;

fn sc_new(n: usize) {
    let mut _cache: SwapCache<usize,String> = SwapCache::new(n);
}

fn lru_new(n: usize) {
    let mut _cache: LruCache<usize,String> = LruCache::new(n);
}


fn sc_puts(n: usize) {
    let mut cache: SwapCache<usize,String> = SwapCache::new(n);
    for key in 0..(n*10) {
        cache.put(key, "test value".to_owned());
    }
}

fn lru_puts(n: usize) {
    let mut cache: LruCache<usize,String> = LruCache::new(n);
    for key in 0..(n*10) {
        cache.put(key, "test value".to_owned());
    }
}


fn sc_put_get(n: usize) {
    let mut cache: SwapCache<usize,String> = SwapCache::new(n);
    for key in 0..(n*10) {
        cache.put(key, "test value".to_owned());
    }

    for key in 0..(n*100) {
        let _val = cache.get(&key);
    }
}

fn lru_put_get(n: usize) {
    let mut cache: LruCache<usize,String> = LruCache::new(n);
    for key in 0..(n*10) {
        cache.put(key, "test value".to_owned());
    }

    for key in 0..(n*100) {
        let _val = cache.get(&key);
    }
}

fn sc_put_get2(n: usize) {
    let mut cache: SwapCache<usize,String> = SwapCache::new(n);
    for key in 0..(n*10) {
        for i in 1..10 {
            cache.put(key * i, "test value".to_owned());
        }

        let _val = cache.get(&key);
    }
}

fn lru_put_get2(n: usize) {
    let mut cache: LruCache<usize,String> = LruCache::new(n);
    for key in 0..(n*10) {
        for i in 1..10 {
            cache.put(key * i, "test value".to_owned());
        }

        let _val = cache.get(&key);
    }
}


fn criterion_benchmark(c: &mut Criterion) {
    
    c.bench_function("SwapCache::new 100", |b| {b.iter(|| sc_new(black_box(100)))});
    c.bench_function("Lru::new 100", |b| {b.iter(|| lru_new(black_box(100)))});

    c.bench_function("SwapCache::puts 100", |b| {b.iter(|| sc_puts(black_box(100)))});
    c.bench_function("Lru::puts 100", |b| {b.iter(|| lru_puts(black_box(100)))});
    
    c.bench_function("SwapCache::put get 100", |b| {b.iter(|| sc_put_get(black_box(100)))});
    c.bench_function("Lru::put get 100", |b| {b.iter(|| lru_put_get(black_box(100)))});
 
    c.bench_function("SwapCache::put get 2 100", |b| {b.iter(|| sc_put_get2(black_box(100)))});
    c.bench_function("Lru::put get 2 100", |b| {b.iter(|| lru_put_get2(black_box(100)))});
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
