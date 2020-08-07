pub mod hosting;

mod serving {
    fn take_order() {}
    fn serve_order() {}
    fn take_payment() {}
}

pub mod adder {
    pub fn add_two(a: i32) -> i32 {
        internal_adder(a, 2)
    }

    fn internal_adder(a: i32, b: i32) -> i32 {
        a + b
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn internal() {
            // Rust’s privacy rules do allow you to test private functions, pp. 255
            assert_eq!(4, internal_adder(2, 2));
        }
    }
}


