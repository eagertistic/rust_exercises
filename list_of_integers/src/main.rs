// Given a list of float numbers, use a vector and return the median (when sorted, the value in the middle position)
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.
// Because float numbers don't add up like 0.1 + 0.2 != 0.3 exactly so Rust requires float numbers to be placed inside a wrapper that makes sure Nan != Nan
use ordered_float::NotNan;
use std::collections::HashMap;
fn main() {
    let mut v = vec![
        NotNan::new(3.7).unwrap(),
        NotNan::new(1.2).unwrap(),
        NotNan::new(9.8).unwrap(),
        NotNan::new(0.5).unwrap(),
        NotNan::new(6.3).unwrap(),
        NotNan::new(4.4).unwrap(),
        NotNan::new(7.1).unwrap(),
        NotNan::new(2.9).unwrap(),
        NotNan::new(8.6).unwrap(),
        NotNan::new(5.0).unwrap(),
    ];
    v.sort();
    let is_even = v.len() % 2;
    let median_index = v.len() / 2;
    match is_even {
        0 => println!(
            "Th median value is {}",
            (&v[median_index - 1] + &v[median_index]) / 2.0
        ),
        _ => println!("The median number is {}", &v[median_index]),
    }

    // Mode
    let mut map = HashMap::new();
    for value in &v {
        let count = map.entry(value).or_insert(0); // Because NotNan implements Ord and Eq, it can be used as a key in a Hashmap 
        *count += 1;
    }
    println!("{map:?}")
}
