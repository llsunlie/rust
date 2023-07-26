use std::{collections::HashMap, hash::Hash};

fn main() {
    /* new */
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);

    /* another new */
    let teams = vec![
        String::from("blue"),
        String::from("yellow")
    ];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    /* owner_ship */
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // println!("map[{}] = {}", field_name, field_value); // error

    /* get values */
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("scores[{}] = {:?}", team_name, score);

    /* for map */
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("scores[{}] = {}", key, value);
    }

    /* update map */
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    scores.insert(String::from("Blue"), 25);

    println!("scores = {:?}", scores);    
    
    /* insert if None */
    let mut scores = HashMap::new();
    
    scores.insert(String::from("Blue"), 10);
    
    scores.entry(String::from("Blue")).or_insert(25);
    scores.entry(String::from("Yellow")).or_insert(50);
    
    println!("scores = {:?}", scores);    

    /* update values with old values */
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("map = {:?}", map);


}
