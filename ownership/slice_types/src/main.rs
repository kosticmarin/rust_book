// The Slice type
//
// Examples from the book in action!
//
// write a function that takes a string and returs the first word
// it finds in that string. If the function doesn't find a space
// in string, the whole string must be one word, so the entire
// string should be returned.

// return an index of the end of the word!
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    dbg!(bytes);
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

// same fn using slices
fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// get second word using slices
fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut prev_i = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' && prev_i != 0 {
            prev_i += 1;
            return &s[prev_i..i];
        } else if item == b' ' {
            prev_i = i;
        }
    }
    &s[..]
}
fn main() {
    let s = String::from("Hello, world");
    let index = first_word(&s);
    dbg!(index);
    //slices before learning slices
    dbg!(&s[0..index]);
    dbg!(first_word_slice(&s));
    dbg!(first_word_slice(&"Bye bye".to_owned()));
    dbg!(second_word(&"Bye bye bye".to_owned()));
}
