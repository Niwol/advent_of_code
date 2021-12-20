use std::{collections::HashMap, fs};

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();

    let lines = data.split('\n');
    let mut stack = Vec::new();
    let mut score = 0;
    let mut scores_v2 = Vec::new();
    let mut score_map_v1 = HashMap::new();
    let mut score_map_v2 = HashMap::new();
    let mut opposite_map = HashMap::new();

    score_map_v1.insert(')', 3);
    score_map_v1.insert(']', 57);
    score_map_v1.insert('}', 1197);
    score_map_v1.insert('>', 25137);

    score_map_v2.insert(')', 1);
    score_map_v2.insert(']', 2);
    score_map_v2.insert('}', 3);
    score_map_v2.insert('>', 4);

    opposite_map.insert('(', ')');
    opposite_map.insert('[', ']');
    opposite_map.insert('{', '}');
    opposite_map.insert('<', '>');
    opposite_map.insert(')', '(');
    opposite_map.insert(']', '[');
    opposite_map.insert('}', '{');
    opposite_map.insert('>', '<');

    for line in lines.clone() {
        stack.clear();
        let mut incomplete_line = true;
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    if *opposite_map.get(&c).unwrap() != stack.pop().unwrap() {
                        score += score_map_v1.get(&c).unwrap();
                        incomplete_line = false;
                        break;
                    }
                }
                _ => panic!("wrong character"),
            }
        }

        if incomplete_line {
            let mut score_v2: u64 = 0;

            while !stack.is_empty() {
                score_v2 *= 5;
                score_v2 += score_map_v2
                    .get(&opposite_map.get(&stack.pop().unwrap()).unwrap())
                    .unwrap()
            }
            scores_v2.push(score_v2);
        }
    }

    println!("V1\nres = {}", score);

    scores_v2.sort();
    println!("V2\nres = {}", scores_v2[scores_v2.len() / 2]);
}
