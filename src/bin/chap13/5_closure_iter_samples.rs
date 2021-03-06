#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

// The function takes ownership of a vector of shoes and a shoe size as parameters.
// It returns a vector containing only shoes of the speci ed size, pp. 316
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // into_iter call to create an iterator that takes ownership of the vector, pp. 317
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);
        // this will compile error, since shoes_in_my_size took ownership of shoes
//        println!("{:?}", shoes);
        assert_eq!(in_my_size, vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ] );
    }
}

fn main() {

}