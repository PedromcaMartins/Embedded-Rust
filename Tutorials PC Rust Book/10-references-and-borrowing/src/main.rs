fn main() {
    let s1: String = String::from("hello");

    let res: usize = calculate_length(&s1);

    println!("The length of {} is {}", s1, res);

    let mut s2: String = s1.clone();
    add_mut_world(&mut s2);
    println!("{s2}");

    let mut s4 = String::from("hello");

    let r1 = &mut s4;
    let r2 = &mut s4;

    // println!("{}, {}", r1, r2); - Err. Can not borrow more than once at a time

    let mut s5 = String::from("hello");

    let r1 = &s5;
    let r2 = &s5;
/*
    let mut r3 = &mut s5; - Err. Can not borrow mut while there are unmut borrows

    println!("{}, {}, {}", r1, r2, r3);
*/

    let mut s5 = String::from("hello");

    let r1 = &s5;
    let r2 = &s5;

    println!("{}, {}", r1, r2);

    // Valid cause no other unmut used
    let mut r3 = &mut s5;
    println!("{}", r3);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

/*
fn add_world(s: &String) {
    s.push_str(", world!"); - Err. s is not mutable
}
*/

fn add_mut_world(s: &mut String) {
    s.push_str(", world!");
}

/*
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
*/

