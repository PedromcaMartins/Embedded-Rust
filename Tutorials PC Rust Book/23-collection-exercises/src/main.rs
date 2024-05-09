mod median;
mod mode;
mod pig_latin;
mod employee_company_list;

fn main() {
    // println!("Hello, world!");

    // let list = vec![1, 5, 4, 5, 4];

    // let median = median::median(&list);
    // match median {
    //     Some(median) => println!("Median: {median}"),
    //     None => println!("List too small for median"),
    // };

    // let mode = mode::mode(&list);
    // println!("The list of modes is: {:?}", mode);

    // let pig_latin = pig_latin::pig_latin("string");
    // println!("Pig latin of string is: {:?}", pig_latin);

    // let pig_latin = pig_latin::pig_latin("astonaut");
    // println!("Pig latin of astonaut is: {:?}", pig_latin);

    // println!("\n");

    employee_company_list::start_console();
}
