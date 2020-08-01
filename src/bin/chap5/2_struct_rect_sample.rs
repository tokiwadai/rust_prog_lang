#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated function
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;
    println!("The area of the rectangle is {} square pixels.",
        area(width1, height1));

    let rect1 = (30, 50);
    println!("The area2 of the rectangle is {} square pixels.",
             area2(rect1));

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area3 of the rectangle is {} square pixels. {:?}",
        area3(&rect1), rect1);

    println!("The area3b of the rectangle is {} square pixels. {:?}",
             rect1.area(), rect1);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // use :: to call associated function
    let rect4 = Rectangle::square(40);
    println!("The area4 of the rectangle is {} square pixels. {:?}",
             rect4.area(), rect4);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}


#[cfg(test)]
mod tests {
    use super::*;
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
        println!("test larger_can_hold_smaller");
        assert!(larger.can_hold(&smaller));
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
        println!("test smaller_cannot_hold_larger");
        assert!(!smaller.can_hold(&larger));
    }

//    #[test]
//    fn another() {
//        panic!("Make this test fail");
//    }
}