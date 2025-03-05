struct Rectangle {
    width: u32,
    height: u32,
}

struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Self {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {value}.");
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {value}.");
        }
        Guess { value }
    }
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(a: usize) -> usize {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {name}")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn greeting_contains_name() {
        let name = "Alex";
        let result = greeting(name);
        assert!(
            result.contains(name),
            "Greeting did not contain name, name value was `{result}`"
        )
    }
    #[test]
    fn it_adds_two() {
        let result = add_two(2);
        assert_eq!(result, 4)
    }
    #[test]
    fn add_two_numbers() -> Result<(), String> {
        let result = add(2, 2);
        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
    #[test]
    fn failing() {
        let fail = false;
        if fail {
            panic!("This test will fail!")
        } else {
        }
    }
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller))
    }
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger))
    }
}
