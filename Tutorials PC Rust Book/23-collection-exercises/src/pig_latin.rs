pub fn pig_latin(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }

    let mut s = String::from(s);
    s.push('-');

    let ch = s.chars().next().unwrap();
    match ch {
        'a' | 'e' | 'i' | 'o' | 'u' => s.push('h'),
        _ => {
            let ch = s.remove(0);
            s.push(ch);
        },
    }

    s.push_str("ay");
    s
}

#[test]
fn test_pig_latin() {
    assert_eq!(pig_latin(""), "");
    assert_eq!(pig_latin("first"), "irst-fay");
    assert_eq!(pig_latin("apple"), "apple-hay");
}