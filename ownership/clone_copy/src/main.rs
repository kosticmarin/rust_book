fn main() {
    // clone
    let s1 = String::from("hello");
    let s2 = s1.clone();

    dbg!(&s1);
    dbg!(&s2);

    // copy
    let x = 5;
    let y = x;

    dbg!(x);
    dbg!(y);

    // ownership and functions

    let s = String::from("hello");
    takes_ownership(s);
    // won't compile > dbg!(s);

    makes_copy(x);

    let mut s = gives_ownership();
    dbg!(&s);
    s = takes_ownership_gives_back(s);
    dbg!(&s);
}

fn takes_ownership(s: String) {
    dbg!("Takes ownership");
    dbg!(&s);
}

fn makes_copy(i: i32) {
    dbg!("Makes copy");
    dbg!(i);
}

fn gives_ownership() -> String {
    dbg!("Gives Owneship");
    let some_string = String::from("Some String");
    some_string
}

fn takes_ownership_gives_back(s: String) -> String {
    dbg!("Takes ownership gives back");
    s
}
