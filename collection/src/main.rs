// Given a list of integers, use a vector and return
// the mean (the average value), median (when sorted,
// the value in the middle position), mode (the value
// that occures most often; a hash map will be helpful
// here) of the list.
use std::collections::{HashMap, HashSet};

fn mean(nums: &[i32]) -> f64 {
    nums.iter().sum::<i32>() as f64 / nums.len() as f64
}

fn median(nums: &mut [i32]) -> i32 {
    nums.sort();
    nums[nums.len() / 2]
}

fn mode(nums: &[i32]) -> i32 {
    let mut occurrences = HashMap::new();
    for &val in nums {
        *occurrences.entry(val).or_insert(0) += 1;
    }

    dbg!(occurrences)
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of zero numbers")
}

// Convert strings to pig latin. The first consonant of
// each word is moved to the end of the word and "ay" is
// added, so "first" becomes "irst-fay". Words that start
// with a vowel have "hay" added to the end instead ("apple"
// becomes "apple-hay". Keep in mind the details about UTF-8
// encoding!
fn consonants() -> HashSet<char> {
    [
        'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'x',
        'z',
    ]
    .iter()
    .cloned()
    .collect()
}

fn pig_latin(word: &str) -> String {
    let c = word.chars().next().unwrap();
    if dbg!(consonants().contains(&c)) {
        let mut pig_word = word[1..].to_owned() + "-";
        pig_word.push(c);
        pig_word + "ay"
    } else {
        word.to_owned() + "-hay"
    }
}

fn main() {
    let mut numbers = vec![42, 1, 36, 34, 76, 387, 43, 1, 43, 54, 2, 3, 43];
    let first = String::from("first");
    let apple = String::from("apple");

    numbers = dbg!(numbers);
    dbg!(mean(&numbers));
    dbg!(median(&mut numbers));
    dbg!(mode(&numbers));
    dbg!(pig_latin(&first));
    dbg!(pig_latin(&apple));
}
