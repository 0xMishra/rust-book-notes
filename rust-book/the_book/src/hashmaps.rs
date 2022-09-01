use std::{collections::HashMap, hash::Hash};

// HashMap<K, V> stores a mapping of keys of type K to values of type V using a hashing function,

pub fn run_hashmaps() {
    // create an empty hashmap
    let mut scores = HashMap::new();

    // inserting pairs in hashmap
    scores.insert(String::from("red"), 10);
    scores.insert(String::from("blue"), 50);
    // Just like vectors, hash maps store their data on the heap. This HashMap has keys of type String and values of type i32. Like vectors, hash maps are homogeneous

    // accessing values in a hashmap
    let team_name = String::from("red");
    let score = scores.get(&team_name); // this mehod returns an Option<&V>
                                        // f there’s no value for that key in the hash map, get will return None

    // iterating over a hashmap
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // hashmap and ownership
    let field_name = String::from("favorite color");
    let field_value = String::from("red");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, as they moved into the map

    // overwriting a value in hashmap
    // If we insert a key and a value into a hash map and then insert that same key with a different value, the value associated with that key will be replaced

    scores.insert("yellow".to_string(), 40);
    scores.insert("yellow".to_string(), 30);
    println!("{:?}", scores);

    // Adding a Key and Value Only If a Key Isn’t Present
    scores.entry(String::from("green")).or_insert(60); // "entry" checks if there is a value associated with the given key if there isn't, it inserts the key and then returns an enum

    //"or_insert" inserts a value for the same key if that key is not already present in the hashmap

    // updating the value based on the old values

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    // By default, HashMap uses a hashing function called SipHash that can provide resistance to Denial of Service (DoS) attacks
}
