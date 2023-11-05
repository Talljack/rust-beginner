use std::io;
use std::collections::HashMap;
use regex::Regex;
use colored::*;
fn main() {
    println!("{}", "Please enter a sentence: ".green());
    let user_input = get_user_input();
    let word_count = get_word_count(user_input);
    for (word, count) in word_count {
        println!("key: {}  count: {}", word.green(), count.to_string().green());
    }
}

fn get_user_input() -> String {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    user_input.trim().to_string()
}

// use split white space, this has a bug, it will count "hello" and "hello," as two different words
// fn get_word_count(text: String) -> HashMap<String, i32> {
//     let mut word_count = HashMap::new();
//     for word in text.split_whitespace() {
//         let count = word_count.entry(word.to_string()).or_insert(0);
//         *count += 1;
//     }
//     word_count
// }

fn get_word_count(text: String) -> HashMap<String, i32> {
    let word_regex = Regex::new(r"(\w+)").unwrap();
    let mut word_count = HashMap::new();
    for (_, [word]) in word_regex.captures_iter(&text).map(|c| c.extract()) {
        let count = word_count.entry(word.to_string()).or_insert(0);
        *count += 1;
    }
    word_count
}