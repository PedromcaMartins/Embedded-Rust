#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn greeting(name: &str) -> String {
    // format!("Hello {}!", name)
    String::from("Hello")
}

pub fn less_than_100(value: i32) {
    if value > 100 {
        panic!(
            "Guess must be less than or equal to 100, got {}", 
            value
        );
    }
}


#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail.");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            height: 7,
            width: 8,
        };
        let smaller = Rectangle {
            height: 1,
            width: 5,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            height: 7,
            width: 8,
        };
        let smaller = Rectangle {
            height: 1,
            width: 5,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was '{}'",
            result
        );
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        less_than_100(200);
    }

    #[test]
    #[ignore = "too expensive to run"]
    fn it_works_again() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal to four"))
        }
    }
}