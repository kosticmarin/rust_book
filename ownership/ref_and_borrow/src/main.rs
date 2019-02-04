fn main() {
    let s = String::from("Hello");
    let len = calc_len(&s);
    dbg!(&s);
    dbg!(len);

    let mut s2 = String::from("Hello");
    dbg!(&s2);
    change(&mut s2);
    dbg!(&s2);
}

// basic ref borrow
fn calc_len(s: &String) -> usize {
    s.len()
}

// &mut ref borrow (only one at a time)
fn change(s: &mut String) {
    s.push_str(", world");
}

// won't compile if it is a &String instead return String owned!!
#[warn(dead_code)]
fn dangle() -> String {
    let s = String::from("hello");
    s // we return a reference to the String, s
} // here, s goes out of score, and is dropped. It mem goes away.
  //Danger!
