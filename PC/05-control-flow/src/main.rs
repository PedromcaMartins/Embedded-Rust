fn main() {

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");


    // outer loop: counting up 0 -> 2
    // inner loop: counting down 10 -> 9
    let mut count: i32 = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining: i32 = 10;

        'counting_down: loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break 'counting_down;
            }
            if count == 2 {
                break 'counting_up count;
            }

            remaining -= 1;
        }

        count += 1;
    };

    println!("End count = {count}");


    let mut countdown = 10;

    while countdown != 0 {
        println!("{countdown}!");

        countdown -= 1;
    }

    println!("LIFTOFF!");


    let a: [usize; 5] = [10, 20, 30, 40, 50];
    let mut index: usize = 0;

    while index < a.len() {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for element in a {
        println!("the value is: {element}");
    }


    for countdown in (1..10).rev() {
        println!("{countdown}!");
    }

    println!("LIFTOFF!");

}
