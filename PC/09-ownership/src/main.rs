fn main() {

    let mut hello_world_append: String = String::from("Hello");

    hello_world_append.push_str(", world!");

    println!("{}", hello_world_append);


    let x: i32 = 5;
    let y: i32 = x;

    let s1: String = String::from("hello");
    let s2: String = s1;

    // println!("{}, world!", s1); - Err
    println!("{}, world!", s2);


    let s1: String = String::from("hello");
    let s2: String = s1.clone();

    println!("s1 = {}, s1 = {}", s1, s2);


    let s = String::from("hello");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);
    println!("3: {}", x);


    let s1 = gives_ownership();
    println!("s1 {s1}");

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    println!("s3 {s3}");
}

fn takes_ownership(some_string: String) {
    println!("1: {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("2: {}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
