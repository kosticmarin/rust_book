// def a struct

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    // static fn
    fn build_user(email: String, username: String) -> User {
        User {
            username,
            email,
            sign_in_count: 0,
            active: false,
        }
    }
}
fn main() {
    let user1 = User {
        username: String::from("johanez"),
        email: String::from("kosticmarin@example.com"),
        active: true,
        sign_in_count: 1,
    };
    dbg!(user1);

    let mut user2 = User {
        username: String::from("user_name"),
        email: String::from("username@example.com"),
        active: true,
        sign_in_count: 2,
    };
    dbg!(&user2);
    user2.active = false;
    dbg!(&user2);
    dbg!(User::build_user(
        String::from("email@example.com"),
        String::from("user")
    ));
}
