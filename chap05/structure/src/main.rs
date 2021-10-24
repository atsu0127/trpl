struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct RGB(u8, u8, u8);

fn main() {
    let user1 = User {
        username: String::from("some"),
        email: String::from("some@example.com"),
        sign_in_count: 0,
        active: false,
    };

    let user2 = User {
        username: String::from("some2"),
        email: String::from("some2@example.com"),
        ..user1
    };

    let black = RGB(0, 0, 0);

    println!("R: {}, G:  {}, B: {}", black.0, black.1, black.2);
}
