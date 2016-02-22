use std::collections::{HashSet, HashMap};
use std::sync::{Arc, Mutex, Barrier};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

struct CounterInner {
    numbers: Vec<i32>,
    threads_data: HashMap<Option<String>, Vec<i32>>
}

impl CounterInner {

    fn new() -> CounterInner {
        CounterInner {
            numbers: Vec::new(),
            threads_data: HashMap::new()
        }
    }

    fn numbers(&self) -> HashSet<i32> {
        self.numbers.iter().cloned().collect::<HashSet<i32>>()
    }

    fn threads(&self) -> usize {
        self.threads_data.len()
    }

    fn thread_numbers(&self, name: &str) -> HashSet<i32> {
        let name = name.to_owned();
        self.threads_data[&Some(name)].iter().cloned().collect::<HashSet<i32>>()
    }

    fn count(&mut self, n: i32) {
        let thread_name = thread::current().name().map(|n| n.to_owned());
        self.threads_data.entry(thread_name).or_insert_with(Vec::new).push(n);
        self.numbers.push(n);
    }
}

#[derive(Clone)]
pub struct Counter {
    inner: Arc<Mutex<CounterInner>>
}

impl Counter {

    pub fn new() -> Counter {
        Counter {
            inner: Arc::new(Mutex::new(CounterInner::new()))
        }
    }

    pub fn threads(&self) -> usize {
        let guard = self.inner.lock().unwrap();
        guard.threads()
    }

    pub fn numbers(&self) -> HashSet<i32> {
        let guard = self.inner.lock().unwrap();
        guard.numbers()
    }

    pub fn thread_numbers(&self, name: &str) -> HashSet<i32> {
        let guard = self.inner.lock().unwrap();
        guard.thread_numbers(name)
    }

    fn count(&self, n: i32) {
        let mut guard = self.inner.lock().unwrap();
        guard.count(n);
    }
}

pub struct ThreadCounter {
    number_of_threads: usize,
    limit: usize
}

impl ThreadCounter {

    pub fn new(number_of_threads: usize, limit: usize) -> ThreadCounter {
        ThreadCounter {
            number_of_threads: number_of_threads,
            limit: limit
        }
    }

    #[allow(unused_must_use)]
    pub fn count_in_threads(&self, counter: &Counter) {
        let barrier = Arc::new(Barrier::new(self.number_of_threads + 1));
        let mut flags = Vec::with_capacity(self.number_of_threads);
        for _ in (0..).take(self.number_of_threads) {
            flags.push(Arc::new(AtomicBool::new(false)));
        }
        flags[0].store(true, Ordering::SeqCst);
        for i in (1..).take(self.number_of_threads) {
            let barrier = barrier.clone();
            let limit = self.limit as i32;
            let start = i as i32;
            let increment = self.number_of_threads as i32;
            let counter = counter.clone();
            let allow_flag = flags[(i-1) % self.number_of_threads].clone();
            let ready_flag = flags[i % self.number_of_threads].clone();
            thread::Builder::new().name(format!("Thread-{}", i)).spawn(
                move || {
                    let mut current = start;
                    while current <= limit {
                        while !allow_flag.load(Ordering::SeqCst) {
                        }
                        allow_flag.store(false, Ordering::SeqCst);
                        counter.count(current);
                        ready_flag.store(true, Ordering::SeqCst);
                        current += increment;
                    }
                    barrier.wait();
                }
            );
        }
        barrier.wait();
    }
}
