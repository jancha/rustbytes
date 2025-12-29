// Rust Bytes Challenge Issue #99 Thread-Safe Counter

use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone)]
struct SafeCounter {
    value: Arc<Mutex<i32>>,
    logs: Arc<Mutex<Vec<String>>>,
    max: i32,
}

impl SafeCounter {
    fn new(max: i32) -> Self {
        let value = Arc::new(Mutex::new(0));
        let logs = Arc::new(Mutex::new(Vec::new()));
        SafeCounter { value, logs, max }
    }
    fn increment(&self, amount: i32) {
        let mut value = self.value.lock().unwrap();
        *value += amount;
        match *value {
            v if *value > self.max => {
                let mut logs = self.logs.lock().unwrap();
                logs.push("Reset occurred".to_string());
                *value %= self.max;
            }
            v if *value < 0 => {
                *value = 0;
            }
            _ => {}
        }
    }
    fn decrement(&self, amount: i32) {
        self.increment(-amount);
    }
    fn read(&self) -> i32 {
        *self.value.lock().unwrap()
    }
    fn get_logs(&self) -> Vec<String> {
        self.logs.lock().unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_thread() {
        let counter = SafeCounter::new(5);
        counter.increment(3);
        assert_eq!(counter.read(), 3);
        counter.increment(3);
        assert_eq!(counter.read(), 1); // Reset after 6 > 5
        assert_eq!(counter.get_logs(), vec!["Reset occurred".to_string()]);
    }

    #[test]
    fn test_multi_thread() {
        let counter = SafeCounter::new(10);
        let counter_clone = counter.clone();
        let handle = thread::spawn(move || {
            for _ in 0..5 {
                counter_clone.increment(1);
            }
        });
        for _ in 0..6 {
            counter.increment(1);
        }
        handle.join().unwrap();
        assert!(counter.read() <= 10); // May reset
    }

    #[test]
    fn test_decrement() {
        let counter = SafeCounter::new(5);
        counter.increment(3);
        counter.decrement(2);
        assert_eq!(counter.read(), 1);
        counter.decrement(2);
        assert_eq!(counter.read(), 0); // No negative
    }
}
