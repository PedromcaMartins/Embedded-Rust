use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("value: {}, spawned thread", i);
        }    
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("value: {}, main thread", i);
    }

    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Vector: {:?}", v);
    });

    handle.join().unwrap();
}