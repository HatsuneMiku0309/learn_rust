use std::collections::HashMap;

pub fn new_hashmap() {
    let mut scores = HashMap::new();
    scores.insert(String::from("test1"),10);
    scores.insert(String::from("test2"), 20);

    println!("[{FILE_NAME}][8] :: {:?}", scores);
}

pub fn vec_to_hashmap() {
    // use `&str` is best, but exception should use `String` when you want have `Ownership` or String function 
    let mut teams: Vec<&str> = Vec::new();
    let blue_team = String::from("blue team");
    teams.push(&blue_team);
    teams.push("red team");

    let initial_scores = vec![10, 50];

    /*
     The type annotation `HashMap<_, _>` is needed here
     because it's possible to `collect` into many different data structures and Rust doesn't know which you want unless you specify.
     
     For the parameters for the key and value types, however, we use `underscores`,
     and Rust can infer the types that the hash map contains based on the types of the data in the vectors.
     */

    // annotation: 註解, 詮釋
    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    println!("[{FILE_NAME}][31] :: {:?}", scores);
}


pub fn get_hashmap() {
    let mut scores = HashMap::new();
    let black_team = String::from("black team");
    scores.insert(String::from("blue team"), 10);
    scores.insert(String::from("yellow team"), 50);
    scores.insert(black_team, 80);

    // println!("{}", black_team); // this fail, becuase Ownership move to scores `HashMap`.

    let team_name = String::from("black team");
    /*
        Here, `score` will have the value that's associated with the black team, and the result will be `Some(&80)`.
        The result is wrapped in `Some` because `get` returns an `Option<&V>`;
        if there's no value for that key in the hash map, `get` will return `None`.
     */
    let score = scores.get(&team_name);
    println!("[{FILE_NAME}][51] :: {:?}", score);
    let mut some_value = 0;
    if let Some(value) = score {
        some_value = *value;
    }
    println!("[{FILE_NAME}][56] :: some_value: {}", some_value);

    for (key, value) in &scores {
        println!("[{FILE_NAME}][59] :: key: {}, value: {}", key, value);
    }
    
}

pub fn cover_hashmap_value() {
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("blue team"), 10);
    scores.insert(String::from("blue team"), 50);

    println!("[{FILE_NAME}][70] :: cover value: {:?}", scores);
}

/*
    If hashmap key is `None`, can't insert value.
    So `or_insert` action when should no `key`.
 */
pub fn only_insert_value_if_key_no_value() {
    let mut scores: HashMap<String, Option<i32>> = HashMap::new();
    scores.insert(String::from("blue team"), Some(10));
    scores.insert(String::from("red team"), None);

    scores.entry(String::from("yellow team")).or_insert(Some(50));
    scores.entry(String::from("blue team")).or_insert(Some(50));
    scores.entry(String::from("red team")).or_insert(Some(50));

    println!("[{FILE_NAME}][86] :: {:#?}", scores);
}


pub fn update_old_value() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    // let mut a = 32;
    // let qaq = &mut a;
    // *qaq = 123;

    println!("[{FILE_NAME}][104] :: {:?}", map);
}

static FILE_NAME: &str = "collections_demo/hashmap_demo.rs";
