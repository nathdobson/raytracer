use std::default::default;
use std::{mem, thread};
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;
use std::time::Duration;
use image::Progress;
use parking_lot::Mutex;
use rcu_clean::BoxRcu;
use cache_padded::CachePadded;
use rayon::current_thread_index;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

pub struct Counter {
    counters: BoxRcu<Vec<Arc<CachePadded<AtomicU64>>>>,
    mutex: Mutex<()>,
}

impl Counter {
    pub fn new() -> Self {
        Counter { counters: BoxRcu::new(vec![]), mutex: Mutex::new(()) }
    }
    fn counter(&self) -> &AtomicU64 {
        let index = current_thread_index().map_or(0, |x| x + 1);
        if let Some(counter) = self.counters.get(index) {
            return counter;
        }
        {
            let lock = self.mutex.lock();
            let mut vec = self.counters.update();
            let new_len = (index + 1).next_power_of_two();
            if vec.len() < new_len {
                vec.resize_with(new_len, default);
            }
        }
        self.counters.get(index).unwrap()
    }
    pub fn inc(&self) {
        let counter = self.counter();
        counter.fetch_add(1, Relaxed);
    }
    pub fn total(&self) -> u64 {
        self.counters.iter().map(|x| {
            x.load(Relaxed)
        }).sum()
    }
}

#[test]
fn test() {
    for _ in 0..100 {
        let counter = Counter::new();
        (0..1000).into_par_iter().for_each(|x| {
            counter.inc();
        });
        assert_eq!(counter.total(), 1000);
    }
}