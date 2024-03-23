use std::io;

fn main() {
    println!("Welcome to the fahrenheit to celcius converter!");

    println!("Input the temperature in fahrenheit: ");

    let mut temp_fahrenheit: String = String::new();

    io::stdin()
        .read_line(&mut temp_fahrenheit)
        .expect("Error reading line");

    let temp_fahrenheit: f64 = temp_fahrenheit
        .trim()
        .parse()
        .expect("Value is not integer");

    let temp_celsius: f64 = ( temp_fahrenheit - 32.0 ) / 1.8;

    println!("Temperature in celsius: {temp_celsius}");
}
