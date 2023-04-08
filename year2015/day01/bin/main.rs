use std::fs;

fn v1() {
    let data = fs::read_to_string("day01/data.txt").unwrap();

    let mut floor = 0;
    data.chars().for_each(|c| {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {}
        };
    });

    println!("Floor {}", floor);
}

fn v2() {
    let data = fs::read_to_string("day01/data.txt").unwrap();

    let mut floor = 0;
    let mut char_num = 0;

    let mut chars_iter = data.chars();

    while floor >= 0 {
        let c = match chars_iter.next() {
            Some(c) => c,
            None => break,
        };

        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        };

        char_num += 1;
    }

    println!("Position {}", char_num);
}

fn main() {
    v1();
    v2();
}
