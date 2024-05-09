use std::{io, vec};

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
        println!("Fibonacci recursive value at index {n} is {result}");
    
        let result: usize = fibonacci_iterative(n);
        println!("Fibonacci iterative value at index {n} is {result}");
        return;
    }
}

fn fibonacci_recursive(n: usize) -> usize {
    if n == 0 || n == 1 {
        return 1;
    }

    return fibonacci_recursive(n - 2) + fibonacci_recursive(n - 1);
}

fn fibonacci_iterative(n: usize) -> usize {
    let mut fibonacci_sequence: Vec<usize> = vec![1, 1];

    for iteration in 2..n {
        let fib_n_prev_2: usize = fibonacci_sequence[iteration - 2];
        let fib_n_prev_1: usize = fibonacci_sequence[iteration - 1];
        let fib_n_current: usize = fib_n_prev_2 + fib_n_prev_1;
        fibonacci_sequence.push(fib_n_current);
    }

    return fibonacci_sequence.pop().expect("Last element of vector is invalid");
}
