fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list {
        if item > largest { largest = item;
        }
    }
    largest
}

// This will Not work!!
fn largest_not_work<T>(list: &[T]) -> T {
//    let mut largest = list[0];
//    for &item in list {
//        // error[E0369]: binary operation `>` cannot be applied to type `&T`
//        // note: `T` might need a bound for `std::cmp::PartialOrd`
//        if item > largest {
//            largest = item;
//        }
//    }
//    largest
    unimplemented!()
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}


/**
    If we change the return type to &T instead of T,
    thereby changing the body of the function to return a reference,
    we wouldn’t need the Clone or Copy trait bounds
    and we could avoid heap allocations, pp. 211
*/
fn largest2<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list); println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list); println!("The largest1 number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest1 char is {}", result);

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest2(&number_list); println!("The largest2 number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest2(&char_list);
    println!("The largest2 char is {}", result);
}