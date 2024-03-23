use std::{io, result};

fn main() {

    println!("Fibonacci sequence!");
        
    let mut n: String = String::new();

    loop {

        println!("Please input a number: ");

        io::stdin()
            .read_line(&mut n)
            .expect("Error reading line");

        let n: usize = match n
            .trim()
            .parse() {
                Ok(n) => n,
                Err(_) => continue,
            };
    
        let result: usize = fibonacci_recursive(n);

        println!("Fibonacci value at index {n} is {result}");
        return;
    }
}

fn fibonacci_recursive(n: usize) -> usize {
    if n == 0 || n == 1 {
        return n;
    }

    return fibonacci_recursive(n - 2) + fibonacci_recursive(n - 1);
}
