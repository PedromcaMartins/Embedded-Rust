fn main() {
    println!("Hello, world!");

    another_function();

    i32_value(5);

    print_labeled_measurement(6, 'm');

    let x = seven();
    println!("The value of x is {x}");
}

fn another_function() {
    println!("Another function.")
}

fn i32_value(x: i32) {
    println!("x = {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn seven() -> i32 {
    7
}


