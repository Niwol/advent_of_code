use std::fs;

fn main() {
    // V1
    let data = fs::read_to_string("data.txt").unwrap();

    let res = data
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|v| v[0] < v[1])
        .count();

    println!("V1 {}", res);

    // V2
    let data = fs::read_to_string("data.txt").unwrap();

    let res = data
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .windows(3)
        .map(|v| v.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|w| w[0] < w[1])
        .count();

    println!("V2 {}", res);
}
