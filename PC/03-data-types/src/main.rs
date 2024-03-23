
use std::io;

fn main() {

    let guess: u32 = "42"
        .parse()
        .expect("Not a number!");

    let guess: u8 = b'a';
    let guess = guess as char;

    println!("Your guess is {guess}");


    let sum = 5 + 10;
    println!("sum = {sum}");

    let difference = 95.5 - 4.3;
    println!("difference = {difference}");

    let product = 4 * 30;
    println!("product = {product}");
    let product: f64 = 4.0 * 30 as f64;
    println!("product = {product}");

    let quotient = 56.7 / 32.2;
    println!("quotient = {quotient}");
    let truncated = -5 / 3;
    println!("truncated = {truncated}");

    let remainder = 43 % 5;
    println!("remainder = {remainder}");


    let tup: (i32, f64, u8) = (600, 6.4, 45);
    println!("tup: {:#?}", tup);

    let (x, y, z) = tup;
    println!("The value of x, y, z is: ({x}, {y}, {z})");

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array a: {:#?}", a);
    let b: [i32; 5] = [1; 5];
    println!("Array b: {:#?}", b);


    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Please enter an array index: ");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line.");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!("The value of the element at index {index} is {element}");

}
