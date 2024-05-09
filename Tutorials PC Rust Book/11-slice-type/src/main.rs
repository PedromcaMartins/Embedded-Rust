fn main() {
    let mut phrase: String = String::from("Hello world!");
    let first_word: &str = first_word_correct(&phrase);
    println!("The first word in the phrase '{}' is {}", phrase, first_word);

    let second_word: &str = second_word_correct(&phrase);

    // phrase.clear(); - Err. second_word is referencing phrase. phrase can't be referenced as mut

    println!("The second word in the phrase '{}' is {}", phrase, second_word);

    let first_word: &str = first_word_even_more_correct(&phrase);
    println!("The first word in the phrase '{}' is {}", phrase, first_word);
}

fn first_word_incorrect(s: &String) -> usize {
    let s_bytes: &[u8] = s.as_bytes();

    for (i , &item) in s_bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_correct(s: &String) -> &str {
    let s_bytes: &[u8] = s.as_bytes();

    for (i , &item) in s_bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn second_word_correct(s: &String) -> &str {
    let s_bytes = s.as_bytes();
    let mut first_word_index: usize = s.len();

    for (i, &item) in s_bytes.iter().enumerate() {
        if item == b' ' {
            if first_word_index == s.len() {
                first_word_index = i;
            } else {
                return &s[first_word_index..i];
            }
        }
    }

    &s[first_word_index..]
}

fn first_word_even_more_correct(s: &str) -> &str {
    let s_bytes: &[u8] = s.as_bytes();

    for (i , &item) in s_bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}