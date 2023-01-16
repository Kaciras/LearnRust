use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    let e = scores.entry(String::from("Yellow"));

    for i in 0..200 {
        scores.insert(i.to_string(), 10);
    }

    e.or_insert(999);
    println!("{:?}", scores);
}
