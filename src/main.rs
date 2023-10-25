#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main () {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("{:?}", user1);

    user1.email = String::from("anotheremail@example.com");

    println!("{:?}", user1);
    println!("{:?}", build_user("aaa", "bbb"));

    let user2 = User {
        email: String::from("ccc"),
        username: String::from("ddd"),
        ..user1
    };

    println!("create user by spread operator {:?}", user2);

    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("block: {:?}", black);
    println!("origin: {:?}", origin);
}

fn build_user(email: &str, username: &str) -> User {
    User {
        email: String::from(email),
        username: String::from(username),
        active: true,
        sign_in_count: 1,
    }
}