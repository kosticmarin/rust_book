// def a struct

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        username: String::from("johanez"),
        email: String::from("kosticmarin@example.com"),
        active: true,
        sign_in_count: 1,
    };
    dbg!(user1);
}
