struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username, active: true, sign_in_count: 1,
    }
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: false,
        sign_in_count: 10,
    };
    println!("user1.email: {}", user1.email);

    let mut user2 = User {
        email: String::from("someone2@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("user2.email: {}", user2.email);

    user2.email = String::from("anotheremail@example.com");
    println!("user2.email: {}", user2.email);

    let user3 = build_user(String::from("someone3@example.com"),
                           String::from("someusername1234"));
    println!("user3.email: {}", user3.email);

    // The syntax .. speci es that the remaining felds not explicitly set
    // should have the same value as the  elds in the given instance.
    let user4 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    println!("user4.email: {}, user4.username: {}, \
        user4.active: {}, user4.sign_in_count: {}",
             user4.email, user4.username, user4.active, user4.sign_in_count);

    let user5 = User {
        email: String::from("another@example.com"),
        active: true,
        ..user1
    };
    println!("user5.email: {}, user5.username: {}, \
        user5.active: {}, user5.sign_in_count: {}",
             user5.email, user5.username, user5.active, user5.sign_in_count);
}