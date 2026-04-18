#![allow(dead_code)]
pub fn add_two(left: u64) -> u64 {
    left + 2
}

fn greet_with_name(name: &str) -> String {
    format!("Merci {name}")
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        let frenchie = greet_with_name("frenchie");
        assert!(frenchie.contains("frenchie"), "Expected greeting to contain 'Frenchie', got {frenchie}");
        assert!(frenchie.contains("Hello"), "Expected greeting to contain 'Hello', got ->  {frenchie}");
    }
    #[test]
    fn rxplore() {
        let result = add_two(8);
        assert_eq!(10, result, "Expected sum to return 10, value was {result}");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let a = Rectangle {
            width: 45,
            height: 60,
        };

        let b = Rectangle {
            width: 41,
            height: 30,
        };

        assert!(a.can_hold(&b));
    }
    #[test]
    fn smaller_cannot_hold_larger() {
        let a = Rectangle {
            width: 45,
            height: 60,
        };

        let smaller = Rectangle {
            width: 41,
            height: 30,
        };

        assert!(!smaller.can_hold(&a));
    }
}
