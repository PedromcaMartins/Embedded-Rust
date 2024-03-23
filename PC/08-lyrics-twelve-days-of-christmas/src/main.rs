const VERSES: [&str; 12] = [
    "Twelve drummers drumming",
    "Eleven pipers piping",
    "Ten lords a-leaping",
    "Nine ladies dancing",
    "Eight maids a-milking",
    "Seven swans a-swimming",
    "Six geese a-laying",
    "Five golden rings",
    "Four calling birds",
    "Three french hens",
    "Two turtle doves and",
    "A partridge in a pear tree"
];

fn main() {

    let number_paragraphs: usize = VERSES.len();
    let last_verse: usize = VERSES.len();

    for paragraph_index in 1..number_paragraphs {
        println!("On the twelfth day of Christmas, my true love sent to me");

        let first_verse: usize = VERSES.len() - paragraph_index;

        for current_verse in first_verse..last_verse {
            println!("{}", VERSES[current_verse]);
        }

        println!();
    }
}
