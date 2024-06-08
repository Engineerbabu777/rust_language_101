

use std::collections::HashMap;

fn main() {
    println!("Hello, world!, HasMaps");

    let mut scores = HashMap::new();

    scores.insert(String::from("red"), 10);
    scores.insert(String::from("Yellow"), 50);

    scores.insert(String::from("Blue"), 25);

    
    scores.entry(String::from("red")).or_insert(50);
    scores.entry(String::from("gray")).or_insert(50);
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
