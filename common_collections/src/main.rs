use std::collections::HashMap;

fn main() {
    // Creating a Vector
    let mut v: Vec<bool> = Vec::new();

    v.push(true);
    let _ = v.get(0);
    v.pop();

    // Macro that creates a new vector which holds the values given
    let v = vec![1, 3, 5];

    for n_ref in &v {
        println!("{n_ref}")
    }

    println!("Third element is {:?}", &v[2]);

    //////////////////////////////////////////////////////////////////

    let mut s = String::new();

    s.push_str("New");

    println!("{s}");

    let hello = String::from("السلام عليكم");

    println!("{}", hello);

    let new_str = s + &hello;

    println!("{}", &new_str[..9]);

    //////////////////////////////////////////////////////////////////

    let mut map = HashMap::new();

    map.insert(String::from("Messi"), 10);
    map.insert(String::from("Ronaldo"), 07);

    map.entry(String::from("Bale")).or_insert(20);

    for (key, value) in &map {
        println!("{}: Jersey Number {}", key, value);
    }

    let text = "Ancara Messi Ancara Messi Ancara Messi Ancara Messi";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
