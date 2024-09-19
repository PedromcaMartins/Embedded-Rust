#![no_std]

pub struct Counter {
    value: u32,
}

impl Counter {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn increment(&mut self) {
        self.value += 1;
    }

    pub fn get_value(&self) -> u32 {
        self.value
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_initial_value() {
        let counter = Counter::new();
        assert_eq!(counter.get_value(), 0);
    }

    #[test]
    fn test_counter_increment() {
        let mut counter = Counter::new();
        counter.increment();
        assert_eq!(counter.get_value(), 1);
    }

    #[test]
    fn test_counter_multiple_increments() {
        let mut counter = Counter::new();
        counter.increment();
        counter.increment();
        counter.increment();
        assert_eq!(counter.get_value(), 3);
    }

    #[test]
    fn test_counter_default() {
        let counter: Counter = Default::default();
        assert_eq!(counter.get_value(), 0);
    }
}
