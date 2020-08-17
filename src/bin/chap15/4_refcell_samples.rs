/** A library to keep track of how close a value is to a maximum value and
    warn when the value is at certain levels, pp. 373
*/
pub trait Messenger {
    fn send(&self, msg: &str);
}

#[derive(Debug)]
pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger, {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!"); }
        else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::cell::RefMut;

    struct MockMessenger {
//        sent_messages: Vec<String>,
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
//                sent_messages: vec![],
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        /** We create a variable one_borrow for the RefMut<T> smart pointer returned from.
            Then we create another mutable borrow in the same way in the variable.
            This makes two mutable references in the same scope, which isn’t allowed.
            When we run the tests for our library, the code will compile without any errors,
            but the test below will fail with panic message:
            already borrowed: BorrowMutError, pp. 376 */
//        fn send(&self, message: &str) {
//            let mut one_borrow: RefMut<Vec<String>> = self.sent_messages.borrow_mut();
//            let mut two_borrow: RefMut<Vec<String>> = self.sent_messages.borrow_mut();
//
//            one_borrow.push(String::from(message));
//            two_borrow.push(String::from(message));
//        }

        fn send(&self, message: &str) {
            /** We can’t modify the to keep track of the messages,
                because the send method takes an immutable reference to self, pp. 374. */
//            self.sent_messages.push(String::from(message));

            /** We call borrow_mut on the RefCell<Vec<String>> in to get a mutable reference
                to the value inside the RefCell<Vec<String>>, which is the vector.
                Then we can call push on the mutable reference to the vector
                to keep track of the messages sent during the test, pp. 375 */
            let mut one_borrow: RefMut<Vec<String>> = self.sent_messages.borrow_mut();
            one_borrow.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker =
            LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
//        assert_eq!(mock_messenger.sent_messages.len(), 1);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}


fn main() { }