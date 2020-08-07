pub fn add_two(a: i32) -> i32 {
    a+2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub fn greeting2(name: &str) -> String {
    String::from("Hello")
}

pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }

    pub fn newB(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value );
        }
        Guess { value }
    }
}

pub fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_adds_two() {
        // Testing Equality with the assert_eq! and assert_ne! Macros, pp. 237
        assert_eq!(4, add_two(2));
        assert_ne!(4, add_two(3));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }

    #[test]
    fn greeting_contains_name2() {
        let result = greeting("Carol");
        // Adding Custom Failure Messages, pp. 239
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", result
        );
    }

    #[test]
    // Checking for Panics with should_panic, pp. 241
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    // To make tests more precise,
    // we can add an optional expected parameter to the attribute.
    // The test harness will make sure that the failure message contains
    // the provided text, pp. 243
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100B() {
        Guess::newB(200);
    }

    // Using Result<T, E> in Tests, pp. 246
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

