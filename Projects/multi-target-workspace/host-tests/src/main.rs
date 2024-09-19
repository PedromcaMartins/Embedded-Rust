use shared_lib::Counter;

fn main() {
    let mut counter = Counter::new();

    println!("Running unit tests on the host machine...");

    for i in 0..5 {
        counter.increment();
        assert_eq!(counter.get_value(), i + 1);
        println!("Test passed for value: {}", i + 1);
    }

    println!("All tests passed!");
}